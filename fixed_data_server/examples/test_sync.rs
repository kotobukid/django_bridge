use anyhow::Result;
use fixed_data_server::sync;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment - try different paths
    if let Err(_) = dotenvy::dotenv() {
        if let Err(_) = dotenvy::from_filename("../.env") {
            dotenvy::from_filename("../../.env").ok();
        }
    }
    
    // Debug: check if API key is loaded
    match env::var("ADMIN_BACKEND_API_KEY") {
        Ok(key) => info!("API key loaded: {}...", &key[..10.min(key.len())]),
        Err(_) => info!("API key not found in environment"),
    }

    info!("Starting sync test...");

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .or_else(|_| -> Result<String, std::env::VarError> {
            let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
            let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
            let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string());
            let db_name = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());
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

    info!("Connected to database");

    // Test sync_push_all
    info!("Testing sync_push_all...");
    match sync::sync_push_all(&pool).await {
        Ok(response) => {
            info!("Push sync successful!");
            info!("  Items received: {}", response.items_received);
            info!("  Items created: {}", response.items_created);
            info!("  Items updated: {}", response.items_updated);
            if !response.errors.is_empty() {
                info!("  Errors: {:?}", response.errors);
            }
        }
        Err(e) => {
            info!("Push sync failed: {}", e);
            info!("Error details: {:?}", e);
            // Try to extract more details if it's a tonic error
            if let Some(source) = e.source() {
                info!("Error source: {:?}", source);
            }
        }
    }

    Ok(())
}