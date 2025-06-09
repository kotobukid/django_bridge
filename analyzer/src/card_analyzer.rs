use crate::raw_card_analyzer::{AnalysisError, RawCardAnalyzer};
use chrono::{DateTime, Utc};
use models::card::CreateCard;
use models::r#gen::django_models::RawCardDb;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;

/// RawCardDb with product_id included
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct RawCardWithProduct {
    pub id: i64,
    pub card_number: String,
    pub name: String,
    pub raw_html: String,
    pub skill_text: String,
    pub life_burst_text: String,
    pub source_url: String,
    pub scraped_at: DateTime<Utc>,
    pub last_analyzed_at: Option<DateTime<Utc>>,
    pub is_analyzed: bool,
    pub analysis_error: String,
    pub product_id: Option<i64>,
}

impl RawCardWithProduct {
    /// Convert to RawCardDb for compatibility
    pub fn to_raw_card_db(&self) -> RawCardDb {
        RawCardDb {
            id: self.id,
            card_number: self.card_number.clone(),
            name: self.name.clone(),
            raw_html: self.raw_html.clone(),
            skill_text: self.skill_text.clone(),
            life_burst_text: self.life_burst_text.clone(),
            source_url: self.source_url.clone(),
            scraped_at: self.scraped_at,
            last_analyzed_at: self.last_analyzed_at,
            is_analyzed: self.is_analyzed,
            analysis_error: self.analysis_error.clone(),
        }
    }
}

/// シンプルなRawCardAnalyzer実装
/// WebAppに依存せずに基本的な解析を行う
pub struct SimpleRawCardAnalyzer;

impl SimpleRawCardAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl RawCardAnalyzer for SimpleRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        self.analyze_with_product_id(raw_card, None).await
    }
}

impl SimpleRawCardAnalyzer {
    pub async fn analyze_with_product_id(
        &self,
        raw_card: &RawCardDb,
        product_id: Option<i64>,
    ) -> Result<CreateCard, AnalysisError> {
        // 基本的なCreateCardを作成
        Ok(CreateCard {
            name: raw_card.name.clone(),
            code: raw_card.card_number.clone(),
            pronunciation: raw_card.name.clone(), // デフォルトで名前を使用
            color: 128,                           // デフォルト無色
            cost: None,
            level: None,
            limit: None,
            limit_ex: None,
            product: product_id.unwrap_or(0) as i32,
            card_type: 0, // TODO: card_type情報を取得
            power: None,
            has_burst: if raw_card.life_burst_text.is_empty() {
                2
            } else {
                1
            },
            skill_text: Some(raw_card.skill_text.clone()),
            burst_text: Some(raw_card.life_burst_text.clone()),
            format: 7, // デフォルトオールスター
            story: None,
            rarity: None,
            timing: None,
            url: Some(raw_card.source_url.clone()),
            feature_bits1: 0, // TODO: 特徴解析を実装
            feature_bits2: 0,
            ex1: None,
        })
    }
}

/// カードをデータベースに保存する機能
pub struct CardRepository {
    pool: Arc<Pool<Postgres>>,
}

impl CardRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }

    pub async fn save_card(
        &self,
        create_card: CreateCard,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let result = sqlx::query(
            r#"
            INSERT INTO wix_card (
                name, code, pronunciation, color, cost, level, "limit", limit_ex,
                product, card_type, power, has_burst, skill_text, burst_text,
                "format", story, rarity, timing, url, feature_bits1, feature_bits2, ex1
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            RETURNING id
            "#
        )
        .bind(&create_card.name)
        .bind(&create_card.code)
        .bind(&create_card.pronunciation)
        .bind(&create_card.color)
        .bind(&create_card.cost)
        .bind(&create_card.level)
        .bind(&create_card.limit)
        .bind(&create_card.limit_ex)
        .bind(&create_card.product)
        .bind(&create_card.card_type)
        .bind(&create_card.power)
        .bind(&create_card.has_burst)
        .bind(&create_card.skill_text)
        .bind(&create_card.burst_text)
        .bind(&create_card.format)
        .bind(&create_card.story)
        .bind(&create_card.rarity)
        .bind(&create_card.timing)
        .bind(&create_card.url)
        .bind(&create_card.feature_bits1)
        .bind(&create_card.feature_bits2)
        .bind(&create_card.ex1)
        .fetch_one(self.pool.as_ref())
        .await?;

        let id: i64 = result.get("id");
        Ok(id)
    }
}

/// RawCardを解析してDBに保存する
pub async fn analyze_and_save_card(
    raw_card: &RawCardDb,
    pool: &Pool<Postgres>,
) -> Result<i64, Box<dyn std::error::Error>> {
    analyze_and_save_card_with_product_id(raw_card, None, pool).await
}

/// RawCardを解析してDBに保存する (product_id指定版)
pub async fn analyze_and_save_card_with_product_id(
    raw_card: &RawCardDb,
    product_id: Option<i64>,
    pool: &Pool<Postgres>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let analyzer = SimpleRawCardAnalyzer::new();

    // RawCardを解析
    let create_card = analyzer
        .analyze_with_product_id(raw_card, product_id)
        .await?;

    // DBに保存
    let card_repo = CardRepository::new(Arc::new(pool.clone()));
    let card_id = card_repo.save_card(create_card).await?;

    // RawCardを解析済みにマーク
    sqlx::query(
        "UPDATE wix_rawcard SET is_analyzed = true, last_analyzed_at = NOW() WHERE id = $1",
    )
    .bind(raw_card.id)
    .execute(pool)
    .await?;

    Ok(card_id)
}

/// 複数のRawCardを一括解析・保存
pub async fn analyze_raw_cards_batch(
    raw_cards: Vec<RawCardDb>,
    pool: &Pool<Postgres>,
) -> Vec<Result<i64, Box<dyn std::error::Error>>> {
    let mut results = Vec::new();

    for raw_card in raw_cards {
        let result = analyze_and_save_card(&raw_card, pool).await;
        results.push(result);
    }

    results
}

/// 複数のRawCardWithProductを一括解析・保存
pub async fn analyze_raw_cards_with_product_batch(
    raw_cards: Vec<RawCardWithProduct>,
    pool: &Pool<Postgres>,
) -> Vec<Result<i64, Box<dyn std::error::Error>>> {
    let mut results = Vec::new();

    for raw_card_with_product in raw_cards {
        let raw_card = raw_card_with_product.to_raw_card_db();
        let result = analyze_and_save_card_with_product_id(
            &raw_card,
            raw_card_with_product.product_id,
            pool,
        )
        .await;
        results.push(result);
    }

    results
}
