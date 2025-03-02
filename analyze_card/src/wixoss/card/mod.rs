mod arts;
mod arts_craft;
mod key;
mod lrig;
mod lrig_assist;
mod piece;
mod piece_relay;
mod resona;
mod resona_craft;
mod signi;
mod spell;
mod spell_craft;

pub use arts::*;
pub use arts_craft::*;
pub use key::*;
pub use lrig::*;
pub use lrig_assist::*;
pub use piece::*;
pub use piece_relay::*;
pub use resona::*;
pub use resona_craft::*;
pub use signi::*;
pub use spell::*;
pub use spell_craft::*;

use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum CardType {
    Lrig,
    LrigAssist,
    Arts,
    Key,
    Signi,
    Spell,
    Resona,
    ArtsCraft,
    ResonaCraft,
    SpellCraft,
    Piece,
    PieceRelay,
    Token,
    Unknown,
}

impl Display for CardType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        let s: &str = match &self {
            CardType::Lrig => "ルリグ",
            CardType::LrigAssist => "ルリグ(アシスト)",
            CardType::Arts => "アーツ",
            CardType::Key => "キー",
            CardType::Signi => "シグニ",
            CardType::Spell => "スペル",
            CardType::Resona => "レゾナ",
            CardType::ArtsCraft => "アーツ(クラフト)",
            CardType::ResonaCraft => "レゾナ(クラフト)",
            CardType::SpellCraft => "スペル(クラフト)",
            CardType::Piece => "ピース",
            CardType::PieceRelay => "ピース(リレー)",
            CardType::Token => "トークン",
            _ => "不明",
        };
        write!(f, "{}", s)
    }
}

pub fn detect_card_type(text: &str) -> CardType {
    #[allow(unreachable_patterns)]
    match text {
        "ルリグ" => CardType::Lrig,
        "アシストルリグ" => CardType::LrigAssist,
        "アーツ" => CardType::Arts,
        "キー" => CardType::Key,
        "シグニ" => CardType::Signi,
        "スペル" => CardType::Spell,
        "レゾナ" => CardType::Resona,
        "アーツ<br />\nクラフト" => CardType::ArtsCraft,
        "シグニ<br />\nクラフト" => CardType::ResonaCraft,
        "スペル<br />\nクラフト" => CardType::SpellCraft,
        "ピース" => CardType::Piece,
        "ピース<br />\nリレー" => CardType::PieceRelay,
        "コイン" => CardType::Token,
        "トークン" => CardType::Token,
        _ => CardType::Unknown,
    }
}
