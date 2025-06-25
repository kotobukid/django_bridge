use crate::repositories::StaticCodeGenerator;
use models::product::{Product, ProductDb};
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct ProductRepository {
    db_connector: Arc<Pool<Postgres>>,
    cache: HashMap<String, i64>,
}

impl ProductRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            db_connector: pool,
            cache: HashMap::new(),
        }
    }

    pub async fn create_cache(&mut self) {
        sqlx::query_as::<_, ProductDb>(
            "SELECT id, name, product_code, url, product_type, sort_asc FROM wix_product",
        )
        .fetch_all(&*self.db_connector)
        .await
        .unwrap()
        .into_iter()
        .for_each(|product| {
            self.cache.insert(product.product_code, product.id);
        })
    }

    pub async fn get_id_by_code(&self, product_code: &str) -> Option<i64> {
        self.cache.get(product_code).cloned()
    }

    pub fn get_all<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Product>, sqlx::Error>> + Send + 'a>> {
        Box::pin(async move {
            let query_future = sqlx::query_as::<_, ProductDb>(
                "SELECT id, name, product_code, url, product_type, sort_asc FROM wix_product ORDER BY sort_asc, id",
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

impl StaticCodeGenerator for ProductRepository {
    async fn code(&self) -> String {
        let lines = self.get_all_as_code().await;
        format!(
            "{}{}{}",
            ProductRepository::headline(lines.len() as i32),
            lines.join("\n"),
            ProductRepository::tail()
        )
    }

    async fn get_all_as_code(&self) -> Vec<String> {
        let products = self.get_all().await.unwrap();
        products
            .into_iter()
            .map(|p| format!(r#"    ({}_u8, "{}", "{}"),"#, p.id, p.product_code, p.name))
            .collect()
    }

    fn headline(length: i32) -> String {
        format!(
            r"pub type ProductStatic = (u8, &'static str, &'static str);
pub const PRODUCT_LIST: &[ProductStatic; {}] = &[
",
            length
        )
    }

    fn tail() -> &'static str {
        "];"
    }
}
