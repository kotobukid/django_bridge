use models::cardtype::CardTypeDb;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CardTypeRepository {
    db: Arc<Pool<Postgres>>,
    cache: HashMap<String, i64>,
}

impl CardTypeRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            db,
            cache: HashMap::new(),
        }
    }

    pub async fn create_cache(&mut self) -> Result<(), sqlx::Error> {
        let res = sqlx::query_as::<_, CardTypeDb>("SELECT * FROM wix_cardtype;")
            .fetch_all(&*self.db)
            .await?;

        res.iter().for_each(|cardtype| {
            self.cache.insert(cardtype.name.clone(), cardtype.id);
        });

        Ok(())
    }

    pub fn find_id_by_name(&self, name: &str) -> Option<i64> {
        self.cache.get(name).cloned()
    }

    pub async fn find_by_code(&self, code: &str) -> Result<i64, sqlx::Error> {
        let id: i64 = sqlx::query_scalar("SELECT id FROM wix_cardtype WHERE code = $1;")
            .bind(code)
            .fetch_one(&*self.db)
            .await?;
        Ok(id)
    }
}
