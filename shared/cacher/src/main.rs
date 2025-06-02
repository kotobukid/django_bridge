use cacher::ProductCacher;
use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::time::Duration;
use webapp::repositories::ProductRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Arc::new(create_db_pool().await?);
    let product_repo = ProductRepository::new(pool.clone());

    let cache_dir = PathBuf::from("../webapp/text_cache");

    let products = product_repo.get_all().await?;
    println!("Found {} products to cache", products.len());

    for product in products {
        let product_cacher = ProductCacher::new(cache_dir.clone(), product);
        
        match product_cacher.cache_all_pages().await {
            Ok(_) => println!("Successfully cached {}", product_cacher),
            Err(e) => eprintln!("Error caching {}: {}", product_cacher, e),
        }
    }

    println!("Caching complete!");
    Ok(())
}

async fn create_db_pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    from_filename("../.env").ok();

    let db_url = build_database_url()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&format!("{}?connect_timeout=5", db_url))
        .await?;

    Ok(pool)
}

fn build_database_url() -> Result<String, Box<dyn std::error::Error>> {
    let host = env::var("DB_HOST")?;
    let port = env::var("DB_PORT")?;
    let user = env::var("DB_USER")?;
    let password = env::var("DB_PASSWORD")?;
    let db_name = env::var("DB_NAME")?;

    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    ))
}