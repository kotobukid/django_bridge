use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use crate::models::{Product, ProductDb};

pub trait IProductRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Product>> + Send + 'a>>;
    // async fn get_by_id(&self, id: i64) -> Option<Card>;
    // async fn add(&self, card: Card);
    // async fn delete(&self, id: i64);
}

pub struct ProductRepository {
    db_connector: Arc<Pool<Postgres>>,
}

impl ProductRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_connector: pool }
    }
}

impl IProductRepository for ProductRepository {
    fn get_all<'a>(&'a self) -> Pin<Box<dyn Future<Output = Vec<Product>> + Send + 'a>> {
        Box::pin(async move {
            let products = sqlx::query_as::<_, ProductDb>("SELECT * FROM wix_product")
                .fetch_all(&*self.db_connector)
                .await
                .unwrap();

            products.into_iter().map(Product::from).collect()
        })
    }

    // async fn get_by_id(&self, id: i64) -> Option<Card> {
    //     todo!()
    // }
    // async fn add(&self, card: Card) {
    //     todo!()
    // }
    // async fn delete(&self, id: i64) {
    //     todo!()
    // }
}