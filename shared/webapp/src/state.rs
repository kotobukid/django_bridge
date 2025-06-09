use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<Pool<Postgres>>,
    pub django_admin_port: u16,
}
