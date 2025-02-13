use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;
// 再エクスポート
pub use crate::gen::django_models::CardDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card(pub CardDb);

impl From<CardDb> for Card {
    fn from(db: CardDb) -> Self {
        Self(db)
    }
}

impl std::ops::Deref for Card {
    type Target = CardDb;

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
    pub fn to_custom_string(&self) -> String {
        match &self.option1 {
            Some(option1) => format!("{}: {} ({})", self.id, self.name, option1),
            None => format!("{}: {}", self.id, self.name),
        }
    }
}


pub trait ICardRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output=Vec<Card>> + Send + 'a>>;
    // async fn get_by_id(&self, id: i64) -> Option<Card>;
    // async fn add(&self, card: Card);
    // async fn delete(&self, id: i64);
}

pub struct CardRepository {
    db_connector: Pool<Postgres>,
}

impl CardRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { db_connector: pool }
    }
}

impl ICardRepository for CardRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output=Vec<Card>> + Send + 'a>> {
        Box::pin(async move {
            let cards = sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card")
                .fetch_all(&self.db_connector)
                .await
                .unwrap();

            cards.into_iter().map(Card::from).collect()
        })
    }

    // async fn get_by_id(&self, id: i64) -> Option<Card> {
    //     todo!()
    // }
    // async fn add(&self, card: Card) {
    //     todo!()
    // }
    // async fn delete(&self, id: i64) {
    //     todo!()
    // }
}
