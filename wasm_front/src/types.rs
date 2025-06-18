use serde::{Deserialize, Serialize};
pub use datapack::CardType;

// Re-export CardExport from datapack as Card
// pub use datapack::CardExport as Card;

#[derive(Debug, Clone, PartialEq)]
pub struct ProductFilter {
    pub selected_products: Vec<u8>,
}

impl ProductFilter {
    pub fn new() -> Self {
        Self {
            selected_products: Vec::new(),
        }
    }
    
    pub fn has_any(&self) -> bool {
        !self.selected_products.is_empty()
    }
    
    pub fn toggle_product(&mut self, product_id: u8) {
        if let Some(pos) = self.selected_products.iter().position(|&p| p == product_id) {
            self.selected_products.remove(pos);
        } else {
            self.selected_products.push(product_id);
        }
    }
    
    pub fn is_selected(&self, product_id: u8) -> bool {
        self.selected_products.contains(&product_id)
    }
    
    pub fn clear_all(&mut self) {
        self.selected_products.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorFilter {
    pub white: bool,
    pub blue: bool,
    pub black: bool,
    pub red: bool,
    pub green: bool,
    pub colorless: bool,
}

impl ColorFilter {
    pub fn new() -> Self {
        Self {
            white: false,
            blue: false,
            black: false,
            red: false,
            green: false,
            colorless: false,
        }
    }

    pub fn to_bits(&self) -> u32 {
        let mut bits = 0u32;
        if self.white {
            bits |= 2;
        }
        if self.blue {
            bits |= 4;
        }
        if self.black {
            bits |= 16;
        }
        if self.red {
            bits |= 8;
        }
        if self.green {
            bits |= 32;
        }
        if self.colorless {
            bits |= 64;
        }
        bits
    }

    pub fn has_any(&self) -> bool {
        self.white || self.blue || self.black || self.red || self.green || self.colorless
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CardTypeFilter {
    pub lrig: bool,
    pub lrig_assist: bool,
    pub arts: bool,
    pub key: bool,
    pub signi: bool,
    pub spell: bool,
    pub resona: bool,
    pub signi_craft: bool,
    pub arts_craft: bool,
    pub resona_craft: bool,
    pub spell_craft: bool,
    pub piece: bool,
    pub piece_relay: bool,
    pub piece_craft: bool,
    pub token: bool,
}

impl CardTypeFilter {
    pub fn new() -> Self {
        Self {
            lrig: false,
            lrig_assist: false,
            arts: false,
            key: false,
            signi: false,
            spell: false,
            resona: false,
            signi_craft: false,
            arts_craft: false,
            resona_craft: false,
            spell_craft: false,
            piece: false,
            piece_relay: false,
            piece_craft: false,
            token: false,
        }
    }

    /// 拡張カードタイプが選択されているかチェック
    pub fn has_extended_selection(&self) -> bool {
        self.key || self.resona || self.signi_craft || self.arts_craft || 
        self.resona_craft || self.spell_craft || self.piece_relay || 
        self.piece_craft || self.token
    }

    pub fn has_any(&self) -> bool {
        self.lrig || self.lrig_assist || self.arts || self.key || self.signi || self.spell 
            || self.resona || self.signi_craft || self.arts_craft || self.resona_craft 
            || self.spell_craft || self.piece || self.piece_relay || self.piece_craft || self.token
    }

    pub fn get_selected_card_types(&self) -> Vec<CardType> {
        let mut selected = Vec::new();
        
        if self.lrig { selected.push(CardType::Lrig); }
        if self.lrig_assist { selected.push(CardType::LrigAssist); }
        if self.arts { selected.push(CardType::Arts); }
        if self.key { selected.push(CardType::Key); }
        if self.signi { selected.push(CardType::Signi); }
        if self.spell { selected.push(CardType::Spell); }
        if self.resona { selected.push(CardType::Resona); }
        if self.signi_craft { selected.push(CardType::SigniCraft); }
        if self.arts_craft { selected.push(CardType::ArtsCraft); }
        if self.resona_craft { selected.push(CardType::ResonaCraft); }
        if self.spell_craft { selected.push(CardType::SpellCraft); }
        if self.piece { selected.push(CardType::Piece); }
        if self.piece_relay { selected.push(CardType::PieceRelay); }
        if self.piece_craft { selected.push(CardType::PieceCraft); }
        if self.token { selected.push(CardType::Token); }
        
        selected
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LevelFilter {
    pub selected_levels: Vec<String>,
}

impl LevelFilter {
    pub fn new() -> Self {
        Self {
            selected_levels: Vec::new(),
        }
    }
    
    pub fn has_any(&self) -> bool {
        !self.selected_levels.is_empty()
    }
    
    pub fn toggle_level(&mut self, level: String) {
        if let Some(pos) = self.selected_levels.iter().position(|l| l == &level) {
            self.selected_levels.remove(pos);
        } else {
            self.selected_levels.push(level);
        }
    }
    
    pub fn is_selected(&self, level: &str) -> bool {
        self.selected_levels.iter().any(|l| l == level)
    }
    
    pub fn clear_all(&mut self) {
        self.selected_levels.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub id: i32,
    pub name: &'static str,
    pub index: u8,
    pub bit: u8,
}

// pub const FEATURES: &[Feature] = &[
//     Feature { id: 1, name: "ダブルクラッシュ", index: 1, bit: 0 },
//     Feature { id: 2, name: "ランサー", index: 1, bit: 1 },
//     Feature { id: 3, name: "アサシン", index: 1, bit: 2 },
//     Feature { id: 4, name: "チャーム", index: 1, bit: 3 },
//     Feature { id: 5, name: "レイヤー", index: 1, bit: 4 },
//     Feature { id: 6, name: "ターン1回", index: 1, bit: 5 },
//     Feature { id: 7, name: "シャドウ", index: 1, bit: 6 },
//     Feature { id: 8, name: "マルチエナ", index: 1, bit: 10 },
//     Feature { id: 9, name: "ライフバースト", index: 1, bit: 11 },
//     Feature { id: 10, name: "エンター", index: 2, bit: 0 },
//     Feature { id: 11, name: "ドライブ", index: 2, bit: 1 },
//     Feature { id: 12, name: "ライズ", index: 2, bit: 2 },
//     Feature { id: 13, name: "ビート", index: 2, bit: 3 },
// ];

// pub fn get_feature_categories() -> Vec<(&'static str, Vec<&'static Feature>)> {
//     vec![
//         ("常在能力", FEATURES.iter().filter(|f| f.id <= 9).collect()),
//         ("トリガー能力", FEATURES.iter().filter(|f| f.id >= 10).collect()),
//     ]
// }
