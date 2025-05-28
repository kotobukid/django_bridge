use std::sync::Arc;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub django_admin_port: u16
}