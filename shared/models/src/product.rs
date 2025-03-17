pub use crate::gen::django_models::{CreateProduct, ProductDb};
use crate::new_type;

new_type!(Product, ProductDb);
