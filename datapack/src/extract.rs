use dotenvy::from_filename;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use webapp::repositories::CardRepository;
use webapp::routers::card_router::CardListJson;

#[tokio::main]
async fn main() {
    from_filename("../.env").ok();

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

    let card_router = CardRepository::new(pool.clone());

    let cards = card_router.get_all_as_card().await;
    let cards: CardListJson = CardListJson::new(cards);
    write_to_file("out/gen.cards.json", cards.to_json().as_str());
    println!("extract")
}

fn write_to_file(file_name: &str, content: &str) {
    let path = Path::new(file_name);

    // 親ディレクトリのパスを取得
    if let Some(parent) = path.parent() {
        // ディレクトリの作成（既に存在していればスキップ）
        fs::create_dir_all(parent).unwrap();
    }

    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
