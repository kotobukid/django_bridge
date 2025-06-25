// 標準ライブラリのインポート
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::Arc;

// 外部クレートのインポート
use clap::Parser;
use dotenvy::from_filename;
use rand::Rng;
use serde_qs as qs;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use url::Url;

// ローカルモジュールのインポート
use models::card::CreateCard;
use webapp::analyze::wixoss::Card;
use webapp::analyze::{
    cache_product_index, collect_card_detail_links, try_mkdir, CardQuery, ProductType,
};
use webapp::repositories::{CardRepository, CardTypeRepository, ProductRepository};

/// データベース接続プールを作成する関数
///
/// .envファイルから環境変数を読み込み、PostgreSQLデータベースへの接続プールを作成します。
async fn create_db() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    let workspace_env = format!(
        "{}/.env",
        env::var("CARGO_WORKSPACE_DIR").unwrap_or_default()
    );
    let env_paths = [
        ".env",                 // カレントディレクトリ
        "../.env",              // 一つ上のディレクトリ
        "../../.env",           // 二つ上のディレクトリ（nested crateの場合）
        workspace_env.as_str(), // CARGO_WORKSPACE_DIRが設定されている場合
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            from_filename(path).ok();
            break;
        }
    }

    // 環境変数からデータベース接続情報を取得
    let db_url = {
        let host = env::var("DB_HOST").map_err(|_| "DB_HOST not found in .env")?;
        let port = env::var("DB_PORT").map_err(|_| "DB_PORT not found in .env")?;
        let user = env::var("DB_USER").map_err(|_| "DB_USER not found in .env")?;
        let password = env::var("DB_PASSWORD").map_err(|_| "DB_PASSWORD not found in .env")?;
        let db_name = env::var("DB_NAME").map_err(|_| "DB_NAME not found in .env")?;

        // PostgreSQL接続URLを構築
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, db_name
        )
    };

    // データベース接続プールを作成
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(format!("{db_url}?connect_timeout=5").as_str())
        .await
        .map_err(|e| format!("データベース接続に失敗しました: {}", e))?;

    Ok(pool)
}

/// カードデータをデータベースに保存する関数
///
/// 指定されたカードデータをデータベースに挿入または更新します。
async fn db(
    pool: Arc<Pool<Postgres>>,
    item: CreateCard,
) -> Result<models::card::Card, sqlx::Error> {
    // カードリポジトリを初期化
    let card_repo = CardRepository::new(pool.clone());
    // カードデータをデータベースに挿入または更新
    card_repo.upsert(item).await
}

/// コマンドライン引数の構造体
#[derive(Parser, Debug)]
struct Args {
    /// 製品タイプ（starter, booster, sp, pr）
    /// 位置引数として指定します
    product_type: String,

    /// 製品コード（オプション）
    /// 製品タイプがpr以外の場合は必須
    code: Option<String>,
}

/// メイン関数
///
/// コマンドライン引数を解析し、指定された製品タイプに基づいてカード情報をスクレイピングし、
/// データベースに保存します。
///
/// # エラー
///
/// 以下の場合にエラーを返します：
/// - 製品コードが必要な製品タイプに対して製品コードが指定されていない場合
/// - キャッシュディレクトリの作成に失敗した場合
/// - カード詳細リンクの収集に失敗した場合
/// - カード情報の解析に失敗した場合
/// - データベース操作に失敗した場合
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // 製品コードを取得（指定がない場合は空文字列）
    let product_code = args.code.unwrap_or_else(|| "".to_string());

    // キャッシュディレクトリを作成
    try_mkdir(Path::new("./text_cache"))
        .map_err(|e| format!("キャッシュディレクトリの作成に失敗しました: {}", e))?;

    // 製品タイプを判定
    let product_type = match args.product_type.to_ascii_lowercase().as_str() {
        "starter" => {
            if product_code.is_empty() {
                return Err("スターターの解析には製品コードが必要です".into());
            }
            ProductType::Starter(product_code)
        }
        "booster" => {
            if product_code.is_empty() {
                return Err("ブースターの解析には製品コードが必要です".into());
            }
            ProductType::Booster(product_code)
        }
        "sp" => {
            if product_code.is_empty() {
                return Err("スペシャルカードの解析には製品コードが必要です".into());
            }
            ProductType::SpecialCard(product_code)
        }
        "pr" => ProductType::PromotionCard,
        _ => {
            return Err(format!("無効な製品タイプです: {}", args.product_type).into());
        }
    };

    // 製品インデックスをキャッシュ
    cache_product_index(&product_type, 1).await?;

    // カード詳細リンクを収集
    let links = collect_card_detail_links(&product_type).await;

    // データベース接続プールを作成し、Arcでラップ
    let pool = Arc::new(create_db().await?);

    // カードタイプリポジトリを初期化
    let card_type_repo = Arc::new(Mutex::new(CardTypeRepository::new(pool.clone())));

    // 製品リポジトリを初期化
    let product_repo = Arc::new(Mutex::new(ProductRepository::new(pool.clone())));

    // リンク収集結果を処理
    let links = links.map_err(|_| "カード詳細リンクの収集に失敗しました")?;
    println!("収集したリンク数: {}", links.len());

    // リンクを順次処理
    for link in links {
        // カード番号を抽出
        let card_no = extract_card_no(&link)
            .ok_or_else(|| format!("カード番号の抽出に失敗しました: {}", link))?;

        // キャッシュディレクトリを指定
        let dir = Path::new("./text_cache/single");
        let cq = CardQuery::new(card_no.clone(), Box::from(dir.to_path_buf()));

        // テキストをキャッシュから取得するか、ダウンロードする
        let text = if cq.check_cache_file_exists() {
            println!("キャッシュが存在します: {card_no}");
            cq.get_cache_text()
        } else {
            println!("キャッシュが見つかりません。ダウンロード中: {card_no}");

            // ランダムな待機時間（1000ms-3000ms）を生成
            let wait_time = rand::thread_rng().gen_range(1000..=3000);
            sleep(Duration::from_millis(wait_time)).await;

            // カード詳細をダウンロード
            // Result<String, AnalyzeError>をOption<String>に変換して型の互換性を確保
            cq.download_card_detail().await.ok()
        };

        // テキストの処理
        match text {
            Some(text) => {
                // HTMLからカード情報を抽出
                match Card::card_from_html(text.as_str()) {
                    Some(card) => {
                        // カードタイプコードを取得
                        let card_type_code = &card.card_type.code();

                        // カードタイプIDをデータベースから取得
                        let card_type_id = card_type_repo
                            .lock()
                            .await
                            .find_by_code(card_type_code)
                            .await
                            .unwrap_or(0); // IDが見つからない場合は0を使用

                        // 製品IDをデータベースから取得
                        let product_id = product_repo
                            .lock()
                            .await
                            .get_id_by_code(&product_type.code())
                            .await
                            .unwrap_or(0); // IDが見つからない場合は0を使用

                        // カードデータをCreateCard形式に変換
                        let mut create_card: CreateCard = card.into();
                        create_card.card_type =
                            card_type_id.to_string().parse::<i32>().map_err(|_| {
                                format!("カードタイプIDの変換に失敗しました: {}", card_type_id)
                            })?;
                        create_card.product = product_id
                            .to_string()
                            .parse::<i32>()
                            .map_err(|_| format!("製品IDの変換に失敗しました: {}", product_id))?;

                        // データベースに保存
                        db(pool.clone(), create_card)
                            .await
                            .map_err(|e| format!("データベースへの保存に失敗しました: {}", e))?;

                        println!("カードを保存しました: {}", card_no);
                    }
                    None => {
                        eprintln!("カードの解析に失敗しました[スキップ]: {}", card_no);
                    }
                }
            }
            None => {
                eprintln!("ダウンロードに失敗しました");
            }
        }
    }

    Ok(())
}

/// URLからカード番号を抽出する関数
///
/// 指定されたURL文字列からカード番号（card_no）を抽出します。
/// URLにカード番号が含まれていない場合はNoneを返します。
///
/// # 引数
///
/// * `url_str` - カード詳細ページのURL文字列
///
/// # 戻り値
///
/// * `Option<String>` - 抽出されたカード番号、または抽出失敗時はNone
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
