use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
// 再エクスポート
pub use crate::gen::django_models::{CardDb, CreateCard, CreateProduct, ProductDb};

macro_rules! new_type {
    ($outer:ident, $inner:ty) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $outer(pub $inner);

        impl From<$inner> for $outer {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }

        impl std::ops::Deref for $outer {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

new_type!(Card, CardDb);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用 -> Derefで不要に
        write!(f, "{}", self.name)
    }
}

impl Card {
    pub fn to_custom_string(&self) -> String {
        self.name.to_string()
    }
}

new_type!(Product, ProductDb);
