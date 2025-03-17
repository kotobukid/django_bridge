use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize)]
pub enum Format {
    AllStar,
    KeySelection,
    DivaSelection,
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::AllStar => write!(f, "all star"),
            Format::KeySelection => write!(f, "key selection"),
            Format::DivaSelection => write!(f, "diva selection"),
        }
    }
}
