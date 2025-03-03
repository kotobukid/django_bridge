use crate::gen::django_models::{CreateKlass, KlassDb, WixCardKlassRel};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::sync::Arc;

pub struct KlassRelRepository {
    db: Arc<Pool<Postgres>>,
    cache: HashMap<CreateKlass, i64>,
}

impl KlassRelRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            db,
            cache: HashMap::new(),
        }
    }

    pub async fn create_cache(&mut self) {
        let klasses = sqlx::query_as::<_, KlassDb>(
            r#"
            SELECT * FROM wix_klass
            "#,
        )
        .fetch_all(&*self.db)
        .await
        .unwrap();

        let mut tree = HashMap::<CreateKlass, i64>::new();
        klasses.iter().for_each(|klass| {
            let cat2: Option<String> = match klass.cat2 {
                Some(ref s) => match s.as_str() {
                    "" => None,
                    _ => Some(s.clone()),
                },
                None => None,
            };
            let cat3: Option<String> = match klass.cat3 {
                Some(ref s) => match s.as_str() {
                    "" => None,
                    _ => Some(s.clone()),
                },
                None => None,
            };

            let ck: CreateKlass = CreateKlass {
                cat1: klass.cat1.clone(),
                cat2,
                cat3,
                // cat1-3で比較するため、sort_ascは0で無視
                sort_asc: 0,
            };
            tree.insert(ck, klass.id);
        });
        self.cache = tree;
    }

    pub fn get_id(&self, klass: &CreateKlass) -> Option<i64> {
        println!("tree: {:?}", self.cache);
        println!("klass: {:?}", klass);
        self.cache.get(klass).cloned()
    }

    pub async fn save(&self, item: WixCardKlassRel) {
        let res = sqlx::query(
            "INSERT INTO wix_card_klass (card_id, klass_id) VALUES ($1, $2) RETURNING *;",
        )
        .bind(item.card_id)
        .bind(item.klass_id)
        .fetch_all(&*self.db)
        .await;
    }
}
