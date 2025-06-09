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
/// 基本的な解析を行う（色情報も含む）
pub struct SimpleRawCardAnalyzer;

impl SimpleRawCardAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    /// HTMLからカード種別を簡易検出する
    /// データベースのwix_cardtypeテーブルのIDに対応
    fn detect_card_type_from_html(&self, html: &str) -> i32 {
        // カード種別セクションを探してそこから種別を抽出
        let card_type_text = if let Some(start) = html.find("<dt>カード種類</dt>") {
            let after_dt = &html[start..];
            if let Some(dd_start) = after_dt.find("<dd>") {
                if let Some(dd_end) = after_dt.find("</dd>") {
                    &after_dt[dd_start + 4..dd_end]
                } else {
                    ""
                }
            } else {
                ""
            }
        } else {
            // フォールバック: 旧形式のHTML構造を試す
            if let Some(start) = html.find("<dt>種類</dt>") {
                let after_dt = &html[start..];
                if let Some(dd_start) = after_dt.find("<dd>") {
                    if let Some(dd_end) = after_dt.find("</dd>") {
                        &after_dt[dd_start + 4..dd_end]
                    } else {
                        ""
                    }
                } else {
                    ""
                }
            } else {
                ""
            }
        };
        
        // println!("DEBUG: Extracted card type text: '{}'", card_type_text);
        
        // 抽出したテキストから種別を判定
        if card_type_text.contains("ルリグ") {
            if card_type_text.contains("アシスト") {
                3 // アシストルリグ (DB ID: 3)
            } else {
                1 // ルリグ (DB ID: 1)
            }
        } else if card_type_text.contains("アーツ") {
            if card_type_text.contains("クラフト") {
                9 // クラフトアーツ (DB ID: 9)
            } else {
                2 // アーツ (DB ID: 2)
            }
        } else if card_type_text.contains("キー") {
            8 // キー (DB ID: 8)
        } else if card_type_text.contains("シグニ") {
            if card_type_text.contains("クラフト") {
                10 // クラフトシグニ (DB ID: 10)
            } else {
                5 // シグニ (DB ID: 5)
            }
        } else if card_type_text.contains("スペル") {
            if card_type_text.contains("クラフト") {
                11 // クラフトスペル (DB ID: 11)
            } else {
                6 // スペル (DB ID: 6)
            }
        } else if card_type_text.contains("レゾナ") {
            if card_type_text.contains("クラフト") {
                14 // クラフトレゾナ (DB ID: 14)
            } else {
                7 // レゾナ (DB ID: 7)
            }
        } else if card_type_text.contains("ピース") {
            if card_type_text.contains("リレー") {
                12 // リレーピース (DB ID: 12)
            } else if card_type_text.contains("クラフト") {
                13 // クラフトピース (DB ID: 13)
            } else {
                4 // ピース (DB ID: 4)
            }
        } else if card_type_text.contains("トークン") {
            15 // トークン (DB ID: 15)
        } else if card_type_text.contains("コイン") {
            16 // コイン (DB ID: 16)
        } else {
            // カード種別セクションが見つからない場合は全体のHTMLから検索（フォールバック）
            // println!("DEBUG: Card type section not found, falling back to full HTML search");
            if html.contains("ルリグ") && !html.contains("ルリグトラッシュ") {
                if html.contains("アシスト") {
                    3 // アシストルリグ (DB ID: 3)
                } else {
                    1 // ルリグ (DB ID: 1)
                }
            } else if html.contains("アーツ") {
                if html.contains("クラフト") {
                    9 // クラフトアーツ (DB ID: 9)
                } else {
                    2 // アーツ (DB ID: 2)
                }
            } else {
                0 // Unknown
            }
        }
    }
    
    /// HTMLから色を検出する
    fn detect_color_from_html(&self, html: &str) -> i32 {
        let mut color = 0;
        
        // <dt>色</dt><dd>○○</dd> パターンで色を検出
        if let Some(color_start) = html.find("<dt>色</dt>") {
            let after_dt = &html[color_start + 11..]; // "<dt>色</dt>"の後
            if let Some(dd_start) = after_dt.find("<dd>") {
                if let Some(dd_end) = after_dt.find("</dd>") {
                    let color_text = &after_dt[dd_start + 4..dd_end];
                    
                    // 各色をチェック
                    if color_text.contains("白") {
                        color |= 1 << 0; // White
                    }
                    if color_text.contains("赤") {
                        color |= 1 << 1; // Red  
                    }
                    if color_text.contains("青") {
                        color |= 1 << 2; // Blue
                    }
                    if color_text.contains("緑") {
                        color |= 1 << 3; // Green
                    }
                    if color_text.contains("黒") {
                        color |= 1 << 4; // Black
                    }
                    if color_text.contains("無") {
                        color = 1 << 7; // Colorless
                    }
                }
            }
        }
        
        // 何も検出されない場合はデフォルトで無色
        if color == 0 {
            color = 1 << 7;
        }
        
        color
    }

    pub async fn analyze_with_product_id(
        &self,
        raw_card: &RawCardDb,
        product_id: Option<i64>,
    ) -> Result<CreateCard, AnalysisError> {
        // HTMLからカード種別を検出
        let card_type = self.detect_card_type_from_html(&raw_card.raw_html);
        // println!("DEBUG: Card {} - Detected card_type: {}", raw_card.card_number, card_type);
        
        // HTMLから色を検出
        let color = self.detect_color_from_html(&raw_card.raw_html);
        // println!("DEBUG: Card {} - Detected color: {}", raw_card.card_number, color);
        
        // 基本的なCreateCardを作成
        Ok(CreateCard {
            name: raw_card.name.clone(),
            code: raw_card.card_number.clone(),
            pronunciation: raw_card.name.clone(), // デフォルトで名前を使用
            color,
            cost: None,
            level: None,
            limit: None,
            limit_ex: None,
            product: product_id.unwrap_or(0) as i32,
            card_type,
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

#[async_trait::async_trait]
impl RawCardAnalyzer for SimpleRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        self.analyze_with_product_id(raw_card, None).await
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
            ON CONFLICT (code) DO UPDATE SET
                name = EXCLUDED.name,
                pronunciation = EXCLUDED.pronunciation,
                color = EXCLUDED.color,
                cost = EXCLUDED.cost,
                level = EXCLUDED.level,
                "limit" = EXCLUDED."limit",
                limit_ex = EXCLUDED.limit_ex,
                product = EXCLUDED.product,
                card_type = EXCLUDED.card_type,
                power = EXCLUDED.power,
                has_burst = EXCLUDED.has_burst,
                skill_text = EXCLUDED.skill_text,
                burst_text = EXCLUDED.burst_text,
                "format" = EXCLUDED."format",
                story = EXCLUDED.story,
                rarity = EXCLUDED.rarity,
                timing = EXCLUDED.timing,
                url = EXCLUDED.url,
                feature_bits1 = EXCLUDED.feature_bits1,
                feature_bits2 = EXCLUDED.feature_bits2,
                ex1 = EXCLUDED.ex1
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
