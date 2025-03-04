pub use crate::gen::django_models::{CardTypeDb, CreateCardType};
use crate::new_type;

new_type!(CardType, CardTypeDb);
