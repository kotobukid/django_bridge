use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
// 再エクスポート
pub use crate::gen::django_models::{CardDb, CreateCard};
use crate::new_type;

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

