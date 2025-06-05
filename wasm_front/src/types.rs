use serde::{Deserialize, Serialize};

// Re-export CardExport from datapack as Card
// pub use datapack::CardExport as Card;

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
        if self.white { bits |= 2; }
        if self.blue { bits |= 4; }
        if self.black { bits |= 16; }
        if self.red { bits |= 8; }
        if self.green { bits |= 32; }
        if self.colorless { bits |= 64; }
        bits
    }

    pub fn has_any(&self) -> bool {
        self.white || self.blue || self.black || self.red || self.green || self.colorless
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    pub id: i32,
    pub name: &'static str,
    pub index: u8,
    pub bit: u8,
}

pub const FEATURES: &[Feature] = &[
    Feature { id: 1, name: "ダブルクラッシュ", index: 1, bit: 0 },
    Feature { id: 2, name: "ランサー", index: 1, bit: 1 },
    Feature { id: 3, name: "アサシン", index: 1, bit: 2 },
    Feature { id: 4, name: "チャーム", index: 1, bit: 3 },
    Feature { id: 5, name: "レイヤー", index: 1, bit: 4 },
    Feature { id: 6, name: "ターン1回", index: 1, bit: 5 },
    Feature { id: 7, name: "シャドウ", index: 1, bit: 6 },
    Feature { id: 8, name: "マルチエナ", index: 1, bit: 10 },
    Feature { id: 9, name: "ライフバースト", index: 1, bit: 11 },
    Feature { id: 10, name: "エンター", index: 2, bit: 0 },
    Feature { id: 11, name: "ドライブ", index: 2, bit: 1 },
    Feature { id: 12, name: "ライズ", index: 2, bit: 2 },
    Feature { id: 13, name: "ビート", index: 2, bit: 3 },
];

pub fn get_feature_categories() -> Vec<(&'static str, Vec<&'static Feature>)> {
    vec![
        ("常在能力", FEATURES.iter().filter(|f| f.id <= 9).collect()),
        ("トリガー能力", FEATURES.iter().filter(|f| f.id >= 10).collect()),
    ]
}