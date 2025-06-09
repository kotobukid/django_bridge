// スクレイピング、キャッシング、HTML解析、データベース保存を行うメインクレート

use std::path::Path;
use std::sync::Arc;

use clap::Parser;
use tokio::sync::Mutex;

use webapp::analyze::{
    cache_product_index, collect_card_detail_links, try_mkdir, ProductType,
};
use webapp::repositories::{CardTypeRepository, ProductRepository};

mod db;
mod scraping;
mod raw_card;

use db::create_database_pool;
use scraping::ScrapingService;

/// コマンドライン引数の構造体
#[derive(Parser, Debug)]
#[command(name = "scraper")]
#[command(about = "WIXOSS カード情報スクレイピングツール")]
struct Args {
    /// 製品タイプ（starter, booster, sp, pr）
    /// 位置引数として指定します
    product_type: String,

    /// 製品コード（オプション）
    /// 製品タイプがpr以外の場合は必須
    code: Option<String>,

    /// キャッシュディレクトリ（デフォルト: ./text_cache）
    #[arg(long, default_value = "./text_cache")]
    cache_dir: String,

    /// ダウンロード間隔の最小値（ミリ秒、デフォルト: 1000）
    #[arg(long, default_value = "1000")]
    min_delay: u64,

    /// ダウンロード間隔の最大値（ミリ秒、デフォルト: 3000）
    #[arg(long, default_value = "3000")]
    max_delay: u64,

    /// 並列処理数（デフォルト: 1）
    #[arg(long, default_value = "1")]
    concurrency: usize,
}

/// メイン関数
///
/// コマンドライン引数を解析し、指定された製品タイプに基づいてカード情報をスクレイピングし、
/// データベースに保存します。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("WIXOSS カード情報スクレイピングツール");
    println!("製品タイプ: {}", args.product_type);
    
    // 製品コードを取得（指定がない場合は空文字列）
    let product_code = args.code.unwrap_or_else(|| "".to_string());
    if !product_code.is_empty() {
        println!("製品コード: {}", product_code);
    }

    // キャッシュディレクトリを作成
    let cache_path = Path::new(&args.cache_dir);
    try_mkdir(cache_path)
        .map_err(|e| format!("キャッシュディレクトリの作成に失敗しました: {}", e))?;
    println!("キャッシュディレクトリ: {}", args.cache_dir);

    // 製品タイプを判定
    let product_type = match args.product_type.to_ascii_lowercase().as_str() {
        "starter" => {
            if product_code.is_empty() {
                return Err("スターターの解析には製品コードが必要です".into());
            }
            ProductType::Starter(product_code)
        },
        "booster" => {
            if product_code.is_empty() {
                return Err("ブースターの解析には製品コードが必要です".into());
            }
            ProductType::Booster(product_code)
        },
        "sp" => {
            if product_code.is_empty() {
                return Err("スペシャルカードの解析には製品コードが必要です".into());
            }
            ProductType::SpecialCard(product_code)
        },
        "pr" => ProductType::PromotionCard,
        _ => {
            return Err(format!("無効な製品タイプです: {}", args.product_type).into());
        }
    };

    // スクレイピングサービスを初期化
    let scraping_service = ScrapingService::new(
        cache_path.to_path_buf(),
        args.min_delay,
        args.max_delay,
        args.concurrency,
    )?;

    // 製品インデックスをキャッシュ
    println!("製品インデックスをキャッシュ中...");
    cache_product_index(&product_type, 1).await?;

    // カード詳細リンクを収集
    println!("カード詳細リンクを収集中...");
    let links = collect_card_detail_links(&product_type).await
        .map_err(|_| "カード詳細リンクの収集に失敗しました")?;
    println!("収集したリンク数: {}", links.len());

    // データベース接続プールを作成
    println!("データベースに接続中...");
    let pool = Arc::new(create_database_pool().await?);

    // リポジトリを初期化
    let card_type_repo = Arc::new(Mutex::new(CardTypeRepository::new(pool.clone())));
    let product_repo = Arc::new(Mutex::new(ProductRepository::new(pool.clone())));

    // スクレイピングを実行
    println!("スクレイピングを開始します...");
    scraping_service.scrape_cards(
        links,
        pool,
        card_type_repo,
        product_repo,
        product_type,
    ).await?;

    println!("スクレイピングが完了しました。");
    Ok(())
}

