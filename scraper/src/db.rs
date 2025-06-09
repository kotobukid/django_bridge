use std::env;
use std::sync::Arc;

use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio::time::Duration;

use models::card::CreateCard;
use webapp::repositories::CardRepository;

/// データベース接続プールを作成する関数
///
/// .envファイルから環境変数を読み込み、PostgreSQLデータベースへの接続プールを作成します。
pub async fn create_database_pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    // ワークスペースルートの.envファイルを読み込む
    // 複数の場所を試行して、最初に見つかったものを使用
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
        .max_connections(10) // scraperでは少し多めに設定
        .acquire_timeout(Duration::from_secs(10))
        .connect(format!("{db_url}?connect_timeout=10").as_str())
        .await
        .map_err(|e| format!("データベース接続に失敗しました: {}", e))?;

    Ok(pool)
}

/// カードデータをデータベースに保存する関数
///
/// 指定されたカードデータをデータベースに挿入または更新します。
pub async fn save_card_to_database(
    pool: Arc<Pool<Postgres>>,
    item: CreateCard,
) -> Result<models::card::Card, sqlx::Error> {
    // カードリポジトリを初期化
    let card_repo = CardRepository::new(pool);
    // カードデータをデータベースに挿入または更新
    card_repo.upsert(item).await
}
