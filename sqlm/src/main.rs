mod gen;
mod models;

use models::{Card, CardDb};
use sqlx::postgres::PgPoolOptions;
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
        .await?;

    let cards: Vec<Card> = sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card")
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|row| Card(row))
        .collect();

    for card in cards {
        println!("{:?}", card);
        println!("{}", card.custom_print());
    }

    Ok(())
}
