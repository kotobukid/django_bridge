mod gen;
mod models;

use models::{Card, CardRepository, ICardRepository};
use serde_json;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
        .await?;

    let card_repo = CardRepository::new(pool);

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

    Ok(())
}
