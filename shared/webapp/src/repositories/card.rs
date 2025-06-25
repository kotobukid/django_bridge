use crate::analyze::wixoss;
use models::card::{Card, CardDb, CreateCard};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

pub trait ICardRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Card>> + Send + 'a>>;
    // async fn get_by_id(&self, id: i64) -> Option<Card>;
    // async fn add(&self, card: Card);
    // async fn delete(&self, id: i64);
}

#[derive(Clone)]
pub struct CardRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl CardRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }

    pub fn get_all<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<CardDb>, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            tokio::time::timeout(
                Duration::from_secs(5),
                sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card")
                    .fetch_all(&*self.db_connector),
            )
            .await
            .map_err(|_| sqlx::Error::PoolTimedOut)?
        })
    }
    pub async fn create_card_full(&self, source: wixoss::Card) -> Result<Card, sqlx::Error> {
        let cc: CreateCard = source.into();
        let res = self.upsert(cc).await;
        let created_card = res?;

        Ok(created_card)
    }

    pub async fn upsert(&self, source: CreateCard) -> Result<Card, sqlx::Error> {
        // まず、指定されたcodeのレコードが存在するか確認
        let existing =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM wix_card WHERE code = $1)")
                .bind(&source.code)
                .fetch_one(&*self.db_connector)
                .await?;

        let card = if existing {
            // レコードが存在する場合はUPDATE
            sqlx::query_as::<_, CardDb>(
                r#"UPDATE wix_card SET
                name = $1,
                pronunciation = $3,
                color = $4,
                cost = $5,
                level = $6,
                "limit" = $7,
                limit_ex = $8,
                power = $9,
                has_burst = $10,
                skill_text = $11,
                burst_text = $12,
                format = $13,
                story = $14,
                rarity = $15,
                url = $16,
                timing = $17,
                card_type = $18,
                product = $19,
                feature_bits1 = $20,
                feature_bits2 = $21,
                burst_bits = $22,
                ex1 = $23
            WHERE code = $2
            RETURNING *"#,
            )
            .bind(source.name)
            .bind(source.code)
            .bind(source.pronunciation)
            .bind(source.color)
            .bind(source.cost)
            .bind(source.level)
            .bind(source.limit)
            .bind(source.limit_ex)
            .bind(source.power)
            .bind(source.has_burst)
            .bind(source.skill_text)
            .bind(source.burst_text)
            .bind(source.format)
            .bind(source.story)
            .bind(source.rarity)
            .bind(source.url)
            .bind(source.timing)
            .bind(source.card_type)
            .bind(source.product)
            .bind(source.feature_bits1)
            .bind(source.feature_bits2)
            .bind(source.burst_bits)
            .bind(source.ex1)
            .fetch_one(&*self.db_connector)
            .await?
        } else {
            // レコードが存在しない場合はINSERT
            sqlx::query_as::<_, CardDb>(
                r#"INSERT INTO wix_card (
                name, code, pronunciation, color, cost, level, "limit",
                limit_ex, power, has_burst, skill_text, burst_text,
                format, story, rarity, url, timing, card_type, product, feature_bits1, feature_bits2, burst_bits, ex1
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23
            ) RETURNING *"#,
            )
            .bind(source.name)
            .bind(source.code)
            .bind(source.pronunciation)
            .bind(source.color)
            .bind(source.cost)
            .bind(source.level)
            .bind(source.limit)
            .bind(source.limit_ex)
            .bind(source.power)
            .bind(source.has_burst)
            .bind(source.skill_text)
            .bind(source.burst_text)
            .bind(source.format)
            .bind(source.story)
            .bind(source.rarity)
            .bind(source.url)
            .bind(source.timing)
            .bind(source.card_type)
            .bind(source.product)
            .bind(source.feature_bits1)
            .bind(source.feature_bits2)
            .bind(source.burst_bits)
            .bind(source.ex1)
            .fetch_one(&*self.db_connector)
            .await?
        };

        Ok(card.into())
    }
    pub fn get_all_as_card<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Card>> + Send + 'a>> {
        Box::pin(async move {
            let cards = sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card")
                .fetch_all(&*self.db_connector)
                .await
                .unwrap();

            cards.into_iter().map(Card::from).collect()
        })
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

pub trait StaticCodeGenerator {
    fn code(&self) -> impl std::future::Future<Output = String> + Send;
    fn get_all_as_code(&self) -> impl std::future::Future<Output = Vec<String>> + Send;

    fn headline(length: i32) -> String;
    fn tail() -> &'static str;
}

impl StaticCodeGenerator for CardRepository {
    async fn code(&self) -> String {
        let lines = self.get_all_as_code().await;
        format!(
            "{}{}{}",
            CardRepository::headline(lines.len() as i32),
            lines.join("\n"),
            CardRepository::tail()
        )
    }

    async fn get_all_as_code(&self) -> Vec<String> {
        // まずすべてのカードを取得
        let cards = sqlx::query_as::<_, CardDb>("SELECT * FROM wix_card where format = 1") // 現在Diva(1)のみ
            .fetch_all(&*self.db_connector)
            .await
            .unwrap(); // エラー処理は適宜修正してください

        // 商品の sort_asc 値を取得するためのマップを作成
        let products = sqlx::query("SELECT id, sort_asc FROM wix_product")
            .fetch_all(&*self.db_connector)
            .await
            .unwrap();

        let product_sort_map: HashMap<i64, i32> = products
            .into_iter()
            .map(|row| {
                let id: i64 = row.get("id");
                let sort_asc: i32 = row.get("sort_asc");
                (id, sort_asc)
            })
            .collect();

        // Klass relationships と bit positions を取得
        let klass_rows = sqlx::query("SELECT id, sort_asc FROM wix_klass ORDER BY sort_asc")
            .fetch_all(&*self.db_connector)
            .await
            .unwrap();

        let klass_bit_map: HashMap<i64, u32> = klass_rows
            .into_iter()
            .enumerate()
            .map(|(index, row)| {
                let id: i64 = row.get("id");
                (id, index as u32)
            })
            .collect();

        // Card-Klass relationships を取得
        let card_klass_rows = sqlx::query("SELECT card_id, klass_id FROM wix_card_klass")
            .fetch_all(&*self.db_connector)
            .await
            .unwrap();

        let mut card_klass_map: HashMap<i64, Vec<i64>> = HashMap::new();
        for row in card_klass_rows {
            let card_id: i64 = row.get("card_id");
            let klass_id: i64 = row.get("klass_id");
            card_klass_map.entry(card_id).or_default().push(klass_id);
        }

        // カードをソート
        let mut cards = cards;
        cards.sort_by(|a, b| {
            let sort_a = product_sort_map.get(&(a.product as i64)).unwrap_or(&999999);
            let sort_b = product_sort_map.get(&(b.product as i64)).unwrap_or(&999999);
            sort_a.cmp(sort_b).then_with(|| a.code.cmp(&b.code))
        });

        cards
            .into_iter()
            .map(|card| {
                let card_obj = Card::from(card);

                // Calculate klass_bits for this card
                let empty_vec = vec![];
                let klass_ids = card_klass_map.get(&card_obj.id).unwrap_or(&empty_vec);
                let mut klass_bits = 0u64;
                for &klass_id in klass_ids {
                    if let Some(&bit_pos) = klass_bit_map.get(&klass_id) {
                        klass_bits |= 1u64 << bit_pos;
                    }
                }

                card_obj.to_rust_code_with_klass_bits(klass_bits)
            })
            .collect()
    }

    fn headline(length: i32) -> String {
        format!(
            r"pub type CardStatic = (i32, &'static str, &'static str, &'static str, u32, &'static str, &'static str, &'static str, &'static str, &'static str, u8, &'static str, &'static str, u8, &'static str, &'static str, u8, u8, u8, i64, i64, u64, i64, &'static str);pub const CARD_LIST: &[CardStatic; {}] = &[",
            length
        )
    }

    fn tail() -> &'static str {
        "];"
    }
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
                sqlx::query_as::<_, OnlyCardName>("SELECT * FROM wix_card")
                    .fetch_all(&*self.db_connector),
            )
            .await
            .map_err(|_| sqlx::Error::PoolTimedOut)?
        })
    }
}
