use crate::models::product::{Product, ProductDb};
use sqlx::{Pool, Postgres};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct ProductRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl ProductRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }

    pub fn get_all<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Product>, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let query_future = sqlx::query_as::<_, ProductDb>(
                "SELECT id, name, product_code, url, product_type, sort_asc FROM wix_product",
            )
            .fetch_all(&*self.db_connector);

            match tokio::time::timeout(Duration::from_secs(5), query_future).await {
                Ok(result) => {
                    result.map(|products| products.into_iter().map(Product::from).collect())
                }
                Err(_) => Err(sqlx::Error::PoolTimedOut),
            }
        })
    }
}
