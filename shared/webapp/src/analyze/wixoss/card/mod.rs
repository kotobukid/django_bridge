mod arts;
mod arts_craft;
mod key;
mod lrig;
mod lrig_assist;
mod piece;
mod piece_craft;
mod piece_relay;
mod resona;
mod resona_craft;
mod signi;
mod signi_craft;
mod spell;
mod spell_craft;

pub use arts::*;
pub use arts_craft::*;
pub use key::*;
pub use lrig::*;
pub use lrig_assist::*;
pub use piece::*;
pub use piece_craft::*;
pub use piece_relay::*;
pub use resona::*;
pub use resona_craft::*;
pub use signi::*;
pub use signi_craft::*;
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
    SigniCraft,
    ArtsCraft,
    ResonaCraft,
    SpellCraft,
    Piece,
    PieceRelay,
    PieceCraft,
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
            CardType::SigniCraft => "シグニ(クラフト)",
            CardType::ArtsCraft => "アーツ(クラフト)",
            CardType::ResonaCraft => "レゾナ(クラフト)",
            CardType::SpellCraft => "スペル(クラフト)",
            CardType::Piece => "ピース",
            CardType::PieceRelay => "ピース(リレー)",
            CardType::PieceCraft => "ピース(クラフト)",
            CardType::Token => "トークン",
            _ => "不明",
        };
        write!(f, "{}", s)
    }
}

impl CardType {
    pub fn code(&self) -> &str {
        match &self {
            CardType::Lrig => "lrig",
            CardType::LrigAssist => "lrig_assist",
            CardType::Arts => "arts",
            CardType::Key => "key",
            CardType::Signi => "signi",
            CardType::Spell => "spell",
            CardType::SpellCraft => "spell_craft",
            CardType::Resona => "resona",
            CardType::SigniCraft => "signi_craft",
            CardType::ArtsCraft => "arts_craft",
            CardType::ResonaCraft => "resona_craft",
            CardType::Piece => "piece",
            CardType::PieceRelay => "piece_relay",
            CardType::PieceCraft => "piece_craft",
            CardType::Token => "token",
            _ => "token",
        }
    }

    /// CardTypeをデータベースIDに変換
    pub fn to_db_id(&self) -> i32 {
        match self {
            CardType::Lrig => 1,
            CardType::LrigAssist => 2,
            CardType::Arts => 3,
            CardType::Key => 4,
            CardType::Signi => 5,
            CardType::Spell => 6,
            CardType::Resona => 7,
            CardType::SigniCraft => 8,
            CardType::ArtsCraft => 9,
            CardType::ResonaCraft => 10,
            CardType::SpellCraft => 11,
            CardType::Piece => 12,
            CardType::PieceRelay => 13,
            CardType::PieceCraft => 14,
            CardType::Token => 15,
            CardType::Unknown => 0,
        }
    }
}

fn remove_tag_and_br(text: &str) -> String {
    text.replace("<br>", "")
        .replace("<br/>", "")
        .replace("<br />", "")
        .replace("\n", "")
}

pub fn detect_card_type(text: &str) -> CardType {
    let text = remove_tag_and_br(text);
    #[allow(unreachable_patterns)]
    match text.as_str() {
        "ルリグ" => CardType::Lrig,
        "アシストルリグ" => CardType::LrigAssist,
        "アーツ" => CardType::Arts,
        "キー" => CardType::Key,
        "シグニ" => CardType::Signi,
        "スペル" => CardType::Spell,
        "レゾナ" => CardType::Resona,
        "シグニクラフト" => CardType::SigniCraft,
        "アーツクラフト" => CardType::ArtsCraft,
        "シグニレゾナクラフト" => CardType::ResonaCraft,
        "スペルクラフト" => CardType::SpellCraft,
        "ピース" => CardType::Piece,
        "ピースリレー" => CardType::PieceRelay,
        "ピースクラフト" => CardType::PieceCraft,
        "コイン" => CardType::Token,
        "トークン" => CardType::Token,
        _ => CardType::Unknown,
    }
}
