pub use crate::gen::django_models::{CreateProduct, ProductDb};
use crate::new_type;

new_type!(Product, ProductDb);

impl Product {
    pub fn cache_path(&self) -> String {
        let sub_dir = match &self.product_type.as_ref() {
            &"bo" => &self.product_code,
            &"st" => &self.product_code,
            &"sp" => &self.product_code,
            &"pr" => "",
            _ => ""
        };
        sub_dir.to_string()
    }
}