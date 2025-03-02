use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use crate::models::{Card, CardDb};

pub trait ICardRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Card>> + Send + 'a>>;
    // async fn get_by_id(&self, id: i64) -> Option<Card>;
    // async fn add(&self, card: Card);
    // async fn delete(&self, id: i64);
}

pub struct CardRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl CardRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }
}

impl ICardRepository for CardRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Card>> + Send + 'a>> {
        Box::pin(async move {
            let cards = sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card")
                .fetch_all(&*self.db_connector)
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

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct OnlyCardName {
    pub name: String,
}

impl Display for OnlyCardName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct OnlyCardNameRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl OnlyCardNameRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }
    pub fn get_all<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<OnlyCardName>, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            tokio::time::timeout(
                Duration::from_secs(5),
                sqlx::query_as::<_, OnlyCardName>("SELECT name FROM wix_card")
                    .fetch_all(&*self.db_connector),
            )
                .await
                .map_err(|_| sqlx::Error::PoolTimedOut)?
        })
    }
}
