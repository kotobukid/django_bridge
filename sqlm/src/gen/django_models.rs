use sqlx;
use chrono;


#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CardDb {
    /// Primary Key
    pub id: i64,
    /// Default: taro, Max length: 256
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub bool1: bool,
    /// Max length: 128
    pub option1: Option<String>,
}


#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct TagDb {
    /// Primary Key
    pub id: i64,
    /// Max length: 128
    pub label: String,
}

