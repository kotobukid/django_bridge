pub mod card;
pub mod product;

#[macro_export]
pub macro_rules! new_type {
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