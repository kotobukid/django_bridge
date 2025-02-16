mod admin_process;
mod gen;
mod models;

use crate::models::OnlyCardNameRepository;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use models::{Card, CardRepository, ICardRepository};
use serde_json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::os::windows::process::CommandExt;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres?connect_timeout=5")
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

    let django_process_handle = Arc::new(Mutex::new(None));

    let app = Router::new()
        .route("/", get(get_index))
        .route("/admin_start", get(admin_process::start_django_server))
        .route("/admin_stop", get(admin_process::stop_django_server))
        .with_state(django_process_handle);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_index() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(
            "<a href=\"/admin_start\">Start Django Server</a>\
        <br />\
        <a href=\"/admin_stop\">Stop Django Server</a>\
    ",
        ),
    )
        .into_response()
}
