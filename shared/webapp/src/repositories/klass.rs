use models::gen::django_models::{CreateKlass, KlassDb, WixCardKlassRel};
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use super::StaticCodeGenerator;

/// クラスリポジトリのエラー型
#[derive(Debug, Error)]
pub enum KlassError {
    #[error("データベースエラー: {0}")]
    Database(#[from] sqlx::Error),
    #[error("クラスが見つかりません: {0:?}")]
    KlassNotFound(CreateKlass),
}

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

    /// キャッシュを作成
    ///
    /// # エラー
    ///
    /// データベースアクセスエラーが発生した場合
    pub async fn create_cache(&mut self) -> Result<(), KlassError> {
        let klasses = sqlx::query_as::<_, KlassDb>(
            r#"
            SELECT * FROM wix_klass
            "#,
        )
        .fetch_all(&*self.db)
        .await?;

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
        Ok(())
    }

    pub fn append_to_cache(&mut self, klass: CreateKlass, id: i64) {
        self.cache.insert(klass, id);
    }

    pub fn get_id_by_create_klass(&self, klass: &CreateKlass) -> Option<i64> {
        println!("tree: {:?}", self.cache);
        println!("klass: {:?}", klass);
        self.cache.get(klass).cloned()
    }

    /// クラスが存在しない場合は作成
    pub async fn create_klass_if_not_exists(
        &mut self,
        klass: CreateKlass,
    ) -> Result<i64, KlassError> {
        if self.check_klass_exists(klass.clone()).await? {
            self.get_id_by_create_klass(&klass)
                .ok_or_else(|| KlassError::KlassNotFound(klass))
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

        Ok(!res.is_empty()) // 存在する場合はtrue
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

        Ok(!found.is_empty()) // 存在する場合はtrue
    }

    /// カードとクラスの関係を保存
    ///
    /// # エラー
    ///
    /// データベースアクセスエラーが発生した場合
    pub async fn save(&self, item: WixCardKlassRel) -> Result<(), KlassError> {
        let exists = self
            .check_rel_exists_by_values(item.card_id, item.klass_id)
            .await?;

        if !exists {
            // 存在しない場合のみINSERT
            sqlx::query(
                "INSERT INTO wix_card_klass (card_id, klass_id) VALUES ($1, $2) RETURNING *;",
            )
            .bind(item.card_id)
            .bind(item.klass_id)
            .fetch_all(&*self.db)
            .await?;
        }

        Ok(())
    }
}

/// KlassRepository for static code generation
#[derive(Clone)]
pub struct KlassRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl KlassRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }

    /// Get all Klass entries ordered by sort_asc
    pub async fn get_all_klasses(&self) -> Result<Vec<KlassDb>, sqlx::Error> {
        sqlx::query_as::<_, KlassDb>("SELECT * FROM wix_klass ORDER BY sort_asc")
            .fetch_all(&*self.db_connector)
            .await
    }

    /// Get card-klass relationships for bit flag generation
    pub async fn get_card_klass_relationships(&self) -> Result<Vec<(i64, Vec<i64>)>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT c.id as card_id, array_agg(k.id ORDER BY k.sort_asc) as klass_ids
            FROM wix_card c
            LEFT JOIN wix_card_klass ck ON c.id = ck.card_id
            LEFT JOIN wix_klass k ON ck.klass_id = k.id
            GROUP BY c.id
            ORDER BY c.id
            "#,
        )
        .fetch_all(&*self.db_connector)
        .await?;

        let mut relationships = Vec::new();
        for row in rows {
            let card_id: i64 = row.get("card_id");
            let klass_ids: Option<Vec<i64>> = row.get("klass_ids");
            let klass_ids = klass_ids.unwrap_or_default();
            // Filter out NULL values that come from LEFT JOIN
            let klass_ids: Vec<i64> = klass_ids.into_iter().filter(|&id| id != 0).collect();
            relationships.push((card_id, klass_ids));
        }

        Ok(relationships)
    }
}

impl StaticCodeGenerator for KlassRepository {
    async fn code(&self) -> String {
        let lines = self.get_all_as_code().await;
        format!(
            "{}{}{}",
            KlassRepository::headline(lines.len() as i32),
            lines.join("\n"),
            KlassRepository::tail()
        )
    }

    async fn get_all_as_code(&self) -> Vec<String> {
        let klasses = self.get_all_klasses().await.unwrap_or_default();
        
        klasses
            .into_iter()
            .enumerate()
            .map(|(index, klass)| {
                let cat2_str = klass.cat2.as_deref().unwrap_or("");
                let cat3_str = klass.cat3.as_deref().unwrap_or("");
                
                format!(
                    r#"({}, "{}", "{}", "{}", {}),"#,
                    klass.id,
                    klass.cat1,
                    cat2_str,
                    cat3_str,
                    index // bit position
                )
            })
            .collect()
    }

    fn headline(length: i32) -> String {
        format!(
            r#"// Klass data: (id, cat1, cat2, cat3, bit_position)
pub type KlassStatic = (i64, &'static str, &'static str, &'static str, u32);
pub const KLASS_LIST: &[KlassStatic; {}] = &["#,
            length
        )
    }

    fn tail() -> &'static str {
        r#"];

// Klass bit flag utilities
pub fn klass_ids_to_bits(klass_ids: &[i64]) -> u64 {
    let mut bits = 0u64;
    for &klass_id in klass_ids {
        if let Some(bit_pos) = get_klass_bit_position(klass_id) {
            bits |= 1u64 << bit_pos;
        }
    }
    bits
}

pub fn get_klass_bit_position(klass_id: i64) -> Option<u32> {
    KLASS_LIST.iter().find(|k| k.0 == klass_id).map(|k| k.4)
}

pub fn has_klass_bits(card_klass_bits: u64, filter_klass_bits: u64) -> bool {
    (card_klass_bits & filter_klass_bits) != 0
}

// Generate bit mask for display labels
pub fn get_klass_display_name(klass_id: i64) -> Option<String> {
    KLASS_LIST.iter().find(|k| k.0 == klass_id).map(|k| {
        if !k.2.is_empty() && !k.3.is_empty() {
            format!("{}:{}/{}", k.1, k.2, k.3)
        } else if !k.2.is_empty() {
            format!("{}:{}", k.1, k.2)
        } else {
            k.1.to_string()
        }
    })
}"#
    }
}
