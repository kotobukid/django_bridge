use std::fmt::{Display, Formatter};
use serde::Serialize;

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