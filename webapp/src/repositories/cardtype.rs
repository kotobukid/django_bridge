use crate::models::cardtype::CardTypeDb;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub struct CardTypeRepository {
    db: Arc<Pool<Postgres>>,
}

impl CardTypeRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self { db }
    }

    pub async fn find_by_code(&self, code: &str) -> Result<i64, sqlx::Error> {
        let cardtype: CardTypeDb =
            sqlx::query_as::<_, CardTypeDb>("SELECT id FROM wix_cardtype WHERE code = $1;")
                .bind(code)
                .fetch_one(&*self.db)
                .await?;
        Ok(cardtype.id)
    }
}
