mod gen;
mod syncdb;

use std::fmt::{write, Display, Formatter};
use std::fs;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::FromRow;
use tokio;
use chrono;

use gen::django_models;

pub struct Card(pub django_models::CardDb);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用
        write!(f, "{}", self.0.name)
    }
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
    //     .await?;
    //
    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool)
    //     .await?;
    //
    // assert_eq!(row.0, 150);

    // let file_path = "../table_definition/wix/models.py"; // Pythonファイルのパス
    // let target_class = "Card"; // 対象のクラス名
    //
    // // Pythonコードをファイルから読み取る
    // let python_source = fs::read_to_string(file_path)?;
    // println!("{}", python_source);



    let card = Card(django_models::CardDb {
        id: 1,
        name: "Taro".into(),
        created_at: chrono::Utc::now(),
        bool1: true,
        option1: Some("jiro".into())
    });

    println!("{}", card);
    Ok(())
}
