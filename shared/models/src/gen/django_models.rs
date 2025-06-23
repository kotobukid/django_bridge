use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct CardDb {
    /// Primary Key
    pub id: i64,
    /// Default: N/A, Max length: 256
    pub name: String,
    /// Default: N/A, Max length: 16
    pub code: String,
    /// Default: N/A, Max length: 128
    pub pronunciation: String,
    pub color: i32,
    /// Max length: 16
    pub cost: Option<String>,
    pub level: Option<i32>,
    pub limit: Option<i32>,
    pub limit_ex: Option<i32>,
    pub product: i32,
    pub card_type: i32,
    /// Max length: 5
    pub power: Option<String>,
    pub has_burst: i32,
    pub skill_text: Option<String>,
    pub burst_text: Option<String>,
    pub format: i32,
    /// Max length: 16
    pub story: Option<String>,
    /// Max length: 8
    pub rarity: Option<String>,
    pub timing: Option<i32>,
    pub url: Option<String>,
    pub feature_bits1: i64,
    pub feature_bits2: i64,
    /// Max length: 256
    pub ex1: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CreateCard {
    /// Default: N/A, Max length: 256
    pub name: String,
    /// Default: N/A, Max length: 16
    pub code: String,
    /// Default: N/A, Max length: 128
    pub pronunciation: String,
    pub color: i32,
    /// Max length: 16
    pub cost: Option<String>,
    pub level: Option<i32>,
    pub limit: Option<i32>,
    pub limit_ex: Option<i32>,
    pub product: i32,
    pub card_type: i32,
    /// Max length: 5
    pub power: Option<String>,
    pub has_burst: i32,
    pub skill_text: Option<String>,
    pub burst_text: Option<String>,
    pub format: i32,
    /// Max length: 16
    pub story: Option<String>,
    /// Max length: 8
    pub rarity: Option<String>,
    pub timing: Option<i32>,
    pub url: Option<String>,
    pub feature_bits1: i64,
    pub feature_bits2: i64,
    /// Max length: 256
    pub ex1: Option<String>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct WixCardUserRel {
    /// Primary Key
    pub id: i64,
    pub card_id: i64,
    pub user_id: i64,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct WixCardKlassRel {
    /// Primary Key
    pub id: i64,
    pub card_id: i64,
    pub klass_id: i64,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct WixCardFeatureRel {
    /// Primary Key
    pub id: i64,
    pub card_id: i64,
    pub feature_id: i64,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct CardTypeDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 8
    pub name: String,
    /// Max length: 32
    pub code: String,
    pub sort_asc: i32,
    pub is_primary: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CreateCardType {
    /// Max length: 8
    pub name: String,
    /// Max length: 32
    pub code: String,
    pub sort_asc: i32,
    pub is_primary: bool,
}


#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct ProductDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 256
    pub name: String,
    /// Max length: 128
    pub product_code: String,
    pub url: Option<String>,
    /// Max length: 2
    pub product_type: String,
    pub sort_asc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CreateProduct {
    /// Max length: 256
    pub name: String,
    /// Max length: 128
    pub product_code: String,
    pub url: Option<String>,
    /// Max length: 2
    pub product_type: String,
    pub sort_asc: i32,
}


#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct KlassDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 5
    pub cat1: String,
    /// Max length: 5
    pub cat2: Option<String>,
    /// Max length: 5
    pub cat3: Option<String>,
    pub sort_asc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CreateKlass {
    /// Max length: 5
    pub cat1: String,
    /// Max length: 5
    pub cat2: Option<String>,
    /// Max length: 5
    pub cat3: Option<String>,
    pub sort_asc: i32,
}


#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct RawCardDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 20
    pub card_number: String,
    /// Max length: 200
    pub name: String,
    pub raw_html: String,
    pub skill_text: String,
    pub life_burst_text: String,
    /// Max length: 500
    pub source_url: String,
    pub scraped_at: DateTime<Utc>,
    pub last_analyzed_at: Option<DateTime<Utc>>,
    pub is_analyzed: bool,
    pub analysis_error: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CreateRawCard {
    /// Max length: 20
    pub card_number: String,
    /// Max length: 200
    pub name: String,
    pub raw_html: String,
    pub skill_text: String,
    pub life_burst_text: String,
    /// Max length: 500
    pub source_url: String,
    pub scraped_at: DateTime<Utc>,
    pub last_analyzed_at: Option<DateTime<Utc>>,
    pub is_analyzed: bool,
    pub analysis_error: String,
}

