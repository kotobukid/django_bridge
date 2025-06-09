use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use webapp::repositories::ProductRepository;

#[tokio::main]
async fn main() {
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

    let product_repository = ProductRepository::new(pool);

    let products = product_repository.get_all().await;
    products
        .iter()
        .for_each(|product| println!("{:?}", product));
}
