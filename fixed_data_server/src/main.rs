use anyhow::Result;
use axum::{routing::{get, post, delete}, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::{env, fs};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod db;

use handlers::{overrides, analyze, import_export};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    let workspace_env = format!(
        "{}/.env",
        env::var("CARGO_WORKSPACE_DIR").unwrap_or_default()
    );
    let env_paths = [
        ".env",                 // カレントディレクトリ
        "../.env",              // 一つ上のディレクトリ
        "../../.env",           // 二つ上のディレクトリ
        workspace_env.as_str(), // CARGO_WORKSPACE_DIRが設定されている場合
    ];

    for path in &env_paths {
        if std::path::Path::new(path).exists() {
            dotenvy::from_filename(path).ok();
            break;
        }
    }

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "fixed_data_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .or_else(|_| -> Result<String, std::env::VarError> {
            // Fallback: construct from individual components
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

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application
    let app = Router::new()
        // Override CRUD endpoints
        .route("/api/overrides", get(overrides::list_overrides))
        .route("/api/overrides", post(overrides::create_or_update_override))
        .route("/api/overrides/:pronunciation", get(overrides::get_override))
        .route("/api/overrides/:pronunciation", delete(overrides::delete_override))
        
        // Analysis endpoints
        .route("/api/analyze/:pronunciation", post(analyze::analyze_card))
        
        // Import/Export endpoints
        .route("/api/export", get(import_export::export_all))
        .route("/api/import", post(import_export::import_data))
        
        // Consistency check
        .route("/api/check-consistency", get(overrides::check_consistency))
        
        // Health check
        .route("/health", get(|| async { "OK" }))
        
        .layer(cors)
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8004));
    info!("Fixed Data Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}