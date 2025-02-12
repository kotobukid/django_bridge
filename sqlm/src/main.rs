mod gen;

use sqlx::postgres::PgPoolOptions;
use std::fmt::{Display, Formatter};
use tokio;

use gen::django_models;

#[derive(Debug)]
pub struct Card(pub django_models::CardDb);

impl From<django_models::CardDb> for Card {
    fn from(db: django_models::CardDb) -> Self {
        Self(db)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用
        write!(f, "{}", self.0.name)
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
        .await?;

    let cards: Vec<Card> =
        sqlx::query_as::<_, django_models::CardDb>("SELECT * FROM wix_card")
            .fetch_all(&pool)
            .await?
            .into_iter().map(|row| Card(row)).collect();

    for card in cards {
        println!("{:?}", card);
    }

    // assert_eq!(row.0, 150);

    // let file_path = "../table_definition/wix/models.py"; // Pythonファイルのパス
    // let target_class = "Card"; // 対象のクラス名
    //
    // // Pythonコードをファイルから読み取る
    // let python_source = fs::read_to_string(file_path)?;
    // println!("{}", python_source);

    Ok(())
}
