use sqlx;
use chrono;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CardDb {
    /// Primary Key
    pub id: u64,
    /// Default: taro, Max length: 256
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub bool1: bool,
    /// Max length: 128
    pub option1: Option<String>,
}
