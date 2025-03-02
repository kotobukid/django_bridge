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
