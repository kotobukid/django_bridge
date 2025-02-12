use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

// 再エクスポート
pub use crate::gen::django_models::CardDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card(pub CardDb);

impl From<CardDb> for Card {
    fn from(db: CardDb) -> Self {
        Self(db)
    }
}

impl std::ops::Deref for Card {
    type Target = CardDb;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Newtype 内の CardDb を参照するために .0 を使用 -> Derefで不要に
        write!(f, "{}", self.name)
    }
}

impl Card {
    pub fn to_custom_string(&self) -> String {
        match &self.option1 {
            Some(option1) => format!("{}: {} ({})", self.id, self.name, option1),
            None => format!("{}: {}", self.id, self.name),
        }
    }
}
