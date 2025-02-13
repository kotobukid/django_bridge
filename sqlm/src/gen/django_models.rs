use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct CardDb {
    /// Primary Key
    pub id: i64,
    /// Default: taro, Max length: 256
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub bool1: bool,
    /// Max length: 128
    pub option1: Option<String>,
    pub info: Option<Value>,
    /// Default: 813-0041, Max length: 10
    pub zip_code: String,

    /// Related field: tags
    /// Note: Check on_delete behavior.
    pub tags: u32,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct TagDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 128
    pub label: String,
}
