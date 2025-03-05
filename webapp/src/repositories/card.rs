use crate::analyze::wixoss;
use crate::models::card::{Card, CardDb, CreateCard};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
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
                product = $19
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
            .fetch_one(&*self.db_connector)
            .await?
        } else {
            // レコードが存在しない場合はINSERT
            sqlx::query_as::<_, CardDb>(
                r#"INSERT INTO wix_card (
                name, code, pronunciation, color, cost, level, "limit",
                limit_ex, power, has_burst, skill_text, burst_text,
                format, story, rarity, url, timing, card_type, product
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16, $17, $18, $19
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
            .fetch_one(&*self.db_connector)
            .await?
        };

        Ok(card.into())
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
                sqlx::query_as::<_, OnlyCardName>("SELECT * FROM wix_card")
                    .fetch_all(&*self.db_connector),
            )
            .await
            .map_err(|_| sqlx::Error::PoolTimedOut)?
        })
    }
}
