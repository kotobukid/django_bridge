use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use rand::Rng;
use serde_qs as qs;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use url::Url;

use webapp::analyze::{CardQuery, ProductType};
use webapp::repositories::{CardTypeRepository, ProductRepository};

use crate::raw_card::RawCardService;

/// スクレイピングサービス
///
/// カード情報のスクレイピング、キャッシング、データベース保存を管理します。
pub struct ScrapingService {
    cache_dir: PathBuf,
    min_delay: u64,
    max_delay: u64,
    #[allow(dead_code)]
    concurrency: usize,
    raw_card_service: RawCardService,
}

impl ScrapingService {
    /// 新しいスクレイピングサービスを作成
    pub fn new(
        cache_dir: PathBuf,
        min_delay: u64,
        max_delay: u64,
        concurrency: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            cache_dir,
            min_delay,
            max_delay,
            concurrency,
            raw_card_service: RawCardService::new()?,
        })
    }

    /// カードリストをスクレイピングして処理
    pub async fn scrape_cards(
        &self,
        links: Vec<String>,
        pool: Arc<Pool<Postgres>>,
        card_type_repo: Arc<Mutex<CardTypeRepository>>,
        product_repo: Arc<Mutex<ProductRepository>>,
        product_type: ProductType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("処理対象のカード数: {}", links.len());

        let mut success_count = 0;
        let mut error_count = 0;
        let mut skip_count = 0;

        // 現在は順次処理（将来的に並列処理を追加可能）
        for (index, link) in links.iter().enumerate() {
            println!("進行状況: {}/{}", index + 1, links.len());

            match self
                .process_single_card(
                    link,
                    pool.clone(),
                    card_type_repo.clone(),
                    product_repo.clone(),
                    &product_type,
                )
                .await
            {
                Ok(ProcessResult::Success) => {
                    success_count += 1;
                    println!("✓ カードを保存しました");
                }
                Ok(ProcessResult::Skipped) => {
                    skip_count += 1;
                    println!("- カードをスキップしました");
                }
                Err(e) => {
                    error_count += 1;
                    eprintln!("✗ エラー: {}", e);
                }
            }

            // 進行状況を定期的に表示
            if (index + 1) % 10 == 0 {
                println!(
                    "中間結果: 成功={}, スキップ={}, エラー={}",
                    success_count, skip_count, error_count
                );
            }
        }

        println!("\n=== 最終結果 ===");
        println!("成功: {}", success_count);
        println!("スキップ: {}", skip_count);
        println!("エラー: {}", error_count);
        println!("合計: {}", links.len());

        Ok(())
    }

    /// 単一のカードを処理
    async fn process_single_card(
        &self,
        link: &str,
        pool: Arc<Pool<Postgres>>,
        card_type_repo: Arc<Mutex<CardTypeRepository>>,
        product_repo: Arc<Mutex<ProductRepository>>,
        product_type: &ProductType,
    ) -> Result<ProcessResult, Box<dyn std::error::Error>> {
        // カード番号を抽出
        let card_no = extract_card_no(link)
            .ok_or_else(|| format!("カード番号の抽出に失敗しました: {}", link))?;

        // キャッシュディレクトリを指定
        let single_cache_dir = self.cache_dir.join("single");
        let cq = CardQuery::new(card_no.clone(), Box::from(single_cache_dir));

        // テキストをキャッシュから取得するか、ダウンロードする
        let text = if cq.check_cache_file_exists() {
            println!("  キャッシュが存在します: {}", card_no);
            cq.get_cache_text()
        } else {
            println!("  キャッシュが見つかりません。ダウンロード中: {}", card_no);

            // ランダムな待機時間を生成
            let wait_time = rand::thread_rng().gen_range(self.min_delay..=self.max_delay);
            sleep(Duration::from_millis(wait_time)).await;

            // カード詳細をダウンロード
            cq.download_card_detail().await.ok()
        };

        // テキストの処理
        match text {
            Some(html_text) => {
                // 製品IDを取得
                let product_id = product_repo
                    .lock()
                    .await
                    .get_id_by_code(&product_type.code())
                    .await;

                // カード名を抽出（HTMLから）
                let card_name = self
                    .raw_card_service
                    .extract_card_name_from_html(&html_text)
                    .unwrap_or_else(|| format!("Unknown Card {}", card_no));

                // CreateRawCardを作成
                let create_raw_card = self.raw_card_service.create_raw_card_from_html(
                    card_no.clone(),
                    card_name,
                    link.to_string(),
                    html_text,
                )?;

                // RawCardをデータベースに保存
                let raw_card_id = self
                    .raw_card_service
                    .save_raw_card_with_product(pool.clone(), create_raw_card, product_id)
                    .await
                    .map_err(|e| format!("RawCardの保存に失敗しました: {}", e))?;

                println!("  ✓ RawCard保存完了 (ID: {})", raw_card_id);
                Ok(ProcessResult::Success)
            }
            None => {
                eprintln!("  ダウンロードに失敗しました: {}", card_no);
                Err("ダウンロード失敗".into())
            }
        }
    }
}

/// 処理結果を表す列挙型
#[derive(Debug)]
enum ProcessResult {
    Success,
    Skipped,
}

/// URLからカード番号を抽出する関数
fn extract_card_no(url_str: &str) -> Option<String> {
    // URLをパース
    let url = Url::parse(url_str).ok()?;

    // クエリ文字列を取得
    let query = url.query()?;

    // クエリパラメータをパース
    let params: HashMap<String, String> = qs::from_str(query).ok()?;

    // card_noパラメータを取得
    params.get("card_no").map(|s| s.to_string())
}
