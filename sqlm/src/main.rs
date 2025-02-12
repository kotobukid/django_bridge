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

impl std::ops::Deref for Card {
    type Target = django_models::CardDb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用 -> Derefで不要に
        write!(f, "{}", self.name)
    }
}

impl Card {
    fn custom_print(&self) -> String {
        match &self.option1 {
            Some(option1) => format!("{}: {} ({})", self.id, self.name, option1),
            None => format!("{}: {}", self.id, self.name),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@192.168.33.10:5432/postgres")
        .await?;

    let cards: Vec<Card> = sqlx::query_as::<_, django_models::CardDb>("SELECT * FROM wix_card")
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
