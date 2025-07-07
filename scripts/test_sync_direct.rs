#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
//! anyhow = "1"
//! tracing = "0.1"
//! tracing-subscriber = "0.3"
//! ```

use anyhow::Result;
use std::env;
use std::path::Path;
use std::process::Command;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Testing direct sync implementation...");

    // 環境変数を設定
    env::set_var("ADMIN_BACKEND_URL", "https://ik1-341-30725.vs.sakura.ne.jp:50051");
    if let Ok(api_key) = env::var("ADMIN_BACKEND_API_KEY") {
        env::set_var("ADMIN_BACKEND_API_KEY", api_key);
    }
    env::set_var("SYNC_CLIENT_ID", "wx_db_local");

    // sync.rsの関数を直接呼び出すためのRustコードを生成
    let test_code = r#"
use fixed_data_server::sync;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // データベース接続
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| -> Result<String, std::env::VarError> {
            let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            let user = std::env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
            let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());
            let db_name = std::env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());
            Ok(format!(
                "postgres://{}:{}@{}:{}/{}",
                user, password, host, port, db_name
            ))
        })
        .expect("DATABASE_URL or DB_* components must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Testing sync_push_all...");
    match sync::sync_push_all(&pool).await {
        Ok(response) => {
            println!("Push sync successful!");
            println!("  Items received: {}", response.items_received);
            println!("  Items created: {}", response.items_created);
            println!("  Items updated: {}", response.items_updated);
            if !response.errors.is_empty() {
                println!("  Errors: {:?}", response.errors);
            }
        }
        Err(e) => {
            println!("Push sync failed: {}", e);
        }
    }

    Ok(())
}
"#;

    // 一時ファイルとして保存
    let test_file = Path::new("/tmp/test_sync_direct.rs");
    std::fs::write(test_file, test_code)?;

    // fixed_data_serverのディレクトリから実行
    let output = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg("/home/kakehashi/RustroverProjects/wx_db/fixed_data_server/Cargo.toml")
        .arg("--bin")
        .arg("test_sync_direct")
        .arg("--")
        .env("RUST_LOG", "info")
        .output()?;

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Error: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}