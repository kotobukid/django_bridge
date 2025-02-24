mod admin_process;
mod gen;
mod models;
mod tokiort;

use crate::models::OnlyCardNameRepository;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use dotenvy;
use models::{Card, CardRepository, ICardRepository};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().ok();
    let db_url = {
        let host = env::var("DB_HOST").unwrap();
        let port = env::var("DB_PORT").unwrap();
        let user = env::var("DB_USER").unwrap();
        let password = env::var("DB_PASSWORD").unwrap();
        let db_name = env::var("DB_NAME").unwrap();
        format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, db_name
        )
    };

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(format!("{db_url}?connect_timeout=5").as_str())
        .await?;

    let pool = Arc::new(pool);

    let card_repo = CardRepository::new(pool.clone());

    let cards: Vec<Card> = card_repo.get_all().await;

    for card in cards {
        println!();
        println!("Debug: {:?}", card);
        println!("Custom: {}", card.to_custom_string());
        println!("JSON: {}", serde_json::to_string(&card).unwrap());
        match &card.info {
            Some(info) => match serde_json::from_str::<serde_json::Value>(&info.to_string()) {
                Ok(value) => {
                    println!("  .Info: {:?}", value);
                }
                Err(e) => {
                    println!("  .Info(parse error): {:?}", e);
                }
            },
            None => {
                println!("Info: None");
            }
        }
    }

    let co_repo = OnlyCardNameRepository::new(pool);

    let names = co_repo.get_all().await;
    for name in names {
        println!("Name: {}", name);
    }
    let a_routers = admin_process::create_admin_portal_router("/admin_operation/");
    let app = Router::new()
        .route("/", get(get_index))
        .nest("/admin_operation/", a_routers.0)
        .nest("/admin_proxy/", a_routers.1)
        .nest("/a_static/", a_routers.2);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_index() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            "<!doctype html><html><title>HOME</title><body><a href=\"/admin_operation/\">manage Django Server</a>\
    </body></html>",
        ),
    )
        .into_response()
}
