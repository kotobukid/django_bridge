use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use webapp::repositories::CardRepository;

const ELASTICSEARCH_ORIGIN: &str = "http://192.168.33.10:9200";
const DOC_TYPE: &str = "card";

#[tokio::main]
async fn main() {
    let workspace_env = format!("{}/.env", env::var("CARGO_WORKSPACE_DIR").unwrap_or_default());
    let env_paths = [
        ".env",                    // カレントディレクトリ
        "../.env",                 // 一つ上のディレクトリ
        "../../.env",              // 二つ上のディレクトリ（nested crateの場合）
        workspace_env.as_str(),    // CARGO_WORKSPACE_DIRが設定されている場合
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            from_filename(path).ok();
            break;
        }
    }

    let db_url = {
        let host = env::var("DB_HOST").expect("DB_HOST not found in .env");
        let port = env::var("DB_PORT").expect("DB_PORT not found in .env");
        let user = env::var("DB_USER").expect("DB_USER not found in .env");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD not found in .env");
        let db_name = env::var("DB_NAME").expect("DB_NAME not found in .env");
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, db_name
        )
    };

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(format!("{db_url}?connect_timeout=5").as_str())
        .await
        .expect("Failed to connect to database");

    let pool = Arc::new(pool);

    let card_repo = CardRepository::new(pool.clone());

    let cards_all = card_repo.get_all_as_card().await;

    // 非同期タスクを作成
    let tasks = cards_all.into_iter().map(|card| {
        let card = card.clone(); // 必要に応じてClone実装
        tokio::spawn(async move {
            // let json_text = serde_json::to_string(&card).unwrap();
            let reqwest_client = reqwest::Client::new();
            let url = format!("{ELASTICSEARCH_ORIGIN}/{DOC_TYPE}/_doc/{}", card.id);
            let response = reqwest_client
                .put(&url)
                .json(&card)
                .send()
                .await
                .unwrap();
            println!("{:?}", response);
        })
    });

    // タスクの並列実行
    futures::future::join_all(tasks).await;
}
