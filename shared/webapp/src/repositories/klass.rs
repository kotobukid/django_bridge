use models::gen::django_models::{CreateKlass, KlassDb, WixCardKlassRel};
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

    pub fn append_to_cache(&mut self, klass: CreateKlass, id: i64) {
        self.cache.insert(klass, id);
    }

    pub fn get_id_by_create_klass(&self, klass: &CreateKlass) -> Option<i64> {
        println!("tree: {:?}", self.cache);
        println!("klass: {:?}", klass);
        self.cache.get(klass).cloned()
    }

    pub async fn create_klass_if_not_exists(
        &mut self,
        klass: CreateKlass,
    ) -> Result<i64, sqlx::Error> {
        if self.check_klass_exists(klass.clone()).await? {
            Ok(self.get_id_by_create_klass(&klass).unwrap())
        } else {
            let id = self.create_klass(klass.clone()).await?;
            self.append_to_cache(klass, id);
            Ok(id)
        }
    }
    async fn check_klass_exists(&self, klass: CreateKlass) -> Result<bool, sqlx::Error> {
        let res = sqlx::query_as::<_, KlassDb>(
            "SELECT * FROM wix_klass WHERE cat1 = $1 AND cat2 = $2 AND cat3 = $3;",
        )
        .bind(klass.cat1)
        .bind(klass.cat2)
        .bind(klass.cat3)
        .fetch_all(&*self.db)
        .await?;

        Ok(res.is_empty())
    }

    async fn create_klass(&mut self, klass: CreateKlass) -> Result<i64, sqlx::Error> {
        let res = sqlx::query_as::<_, KlassDb>(
            "INSERT INTO wix_klass (cat1, cat2, cat3, sort_asc) VALUES ($1, $2, $3, $4) RETURNING *;",
        ).bind(klass.cat1)
            .bind(klass.cat2).bind(klass.cat3).bind(klass.sort_asc)
            .fetch_one(&*self.db).await?;

        self.append_to_cache(
            CreateKlass {
                cat1: res.cat1,
                cat2: res.cat2,
                cat3: res.cat3,
                sort_asc: 0,
            },
            res.id,
        );

        Ok(res.id)
    }

    pub async fn check_rel_exists_by_values(
        &self,
        card_id: i64,
        klass_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let found = sqlx::query_as::<_, WixCardKlassRel>(
            "SELECT * FROM wix_card_klass WHERE card_id = $1 AND klass_id = $2;",
        )
        .bind(card_id)
        .bind(klass_id)
        .fetch_all(&*self.db)
        .await?;

        Ok(found.is_empty())
    }

    pub async fn save(&self, item: WixCardKlassRel) {
        match self
            .check_rel_exists_by_values(item.card_id, item.klass_id)
            .await
        {
            Ok(true) => (),
            Ok(false) => {
                let _ = sqlx::query(
                    "INSERT INTO wix_card_klass (card_id, klass_id) VALUES ($1, $2) RETURNING *;",
                )
                .bind(item.card_id)
                .bind(item.klass_id)
                .fetch_all(&*self.db)
                .await;
            }
            _ => (),
        }
    }
}
