use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;
use tokio;
use chrono;

#[derive(FromRow)]
struct Card {
    id: i64,
    name: String,
    created_at: chrono::DateTime<chrono::Utc> ,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
