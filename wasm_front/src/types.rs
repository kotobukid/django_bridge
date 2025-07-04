pub use datapack::CardType;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq)]
pub struct TimingFilter {
    pub main_phase: bool,
    pub attack_phase: bool,
    pub spell_cutins: bool,
}

impl TimingFilter {
    pub fn new() -> Self {
        Self {
            main_phase: false,
            attack_phase: false,
            spell_cutins: false,
        }
    }

    pub fn has_any(&self) -> bool {
        self.main_phase || self.attack_phase || self.spell_cutins
    }

    /// Check if a timing value matches the current filter
    /// Multiple filters are treated as AND conditions
    pub fn matches(&self, timing: u8) -> bool {
        if !self.has_any() {
            return true; // No filter applied
        }

        // Based on analyzer mapping:
        // アタックフェイズ = 1
        // アタックフェイズスペルカットイン = 2  
        // メインフェイズ = 4
        // メインフェイズアタックフェイズ = 8
        // メインフェイズアタックフェイズスペルカットイン = 16
        // メインフェイズスペルカットイン = 32

        // Check which phases the timing value supports
        let timing_has_main = match timing {
            4 | 8 | 16 | 32 => true,
            _ => false,
        };
        
        let timing_has_attack = match timing {
            1 | 2 | 8 | 16 => true,
            _ => false,
        };
        
        let timing_has_spell_cutins = match timing {
            2 | 16 | 32 => true,
            _ => false,
        };

        // AND logic: all selected phases must be supported by the timing
        if self.main_phase && !timing_has_main {
            return false;
        }
        if self.attack_phase && !timing_has_attack {
            return false;
        }
        if self.spell_cutins && !timing_has_spell_cutins {
            return false;
        }

        // If we reach here, all selected phases are supported
        timing > 0
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
    pub coin: bool,
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
            coin: false,
        }
    }

    /// 拡張カードタイプが選択されているかチェック（動的に判定）
    pub fn has_extended_selection(&self) -> bool {
        // 動的に拡張カードタイプを確認
        self.is_selected_by_code("key")
            || self.is_selected_by_code("resona")
            || self.is_selected_by_code("signi_craft")
            || self.is_selected_by_code("arts_craft")
            || self.is_selected_by_code("resona_craft")
            || self.is_selected_by_code("spell_craft")
            || self.is_selected_by_code("piece_relay")
            || self.is_selected_by_code("piece_craft")
            || self.is_selected_by_code("token")
            || self.is_selected_by_code("coin")
    }

    /// 特定のコードのカードタイプが選択されているかチェック
    pub fn is_selected_by_code(&self, code: &str) -> bool {
        match code {
            "lrig" => self.lrig,
            "lrig_assist" => self.lrig_assist,
            "arts" => self.arts,
            "key" => self.key,
            "signi" => self.signi,
            "spell" => self.spell,
            "resona" => self.resona,
            "signi_craft" => self.signi_craft,
            "arts_craft" => self.arts_craft,
            "resona_craft" => self.resona_craft,
            "spell_craft" => self.spell_craft,
            "piece" => self.piece,
            "piece_relay" => self.piece_relay,
            "piece_craft" => self.piece_craft,
            "token" => self.token,
            "coin" => self.coin,
            _ => false,
        }
    }

    /// 選択されたカードタイプの中に拡張タイプがあるかチェック（生成されたデータを使用）
    #[allow(unused)]
    pub fn has_extended_selection_dynamic(&self) -> bool {
        use datapack::gen::card_types::EXTENDED_CARD_TYPES;

        EXTENDED_CARD_TYPES
            .iter()
            .any(|&code| self.is_selected_by_code(code))
    }

    /// 特定のコードのカードタイプの選択状態を設定
    pub fn set_by_code(&mut self, code: &str, value: bool) {
        match code {
            "lrig" => self.lrig = value,
            "lrig_assist" => self.lrig_assist = value,
            "arts" => self.arts = value,
            "key" => self.key = value,
            "signi" => self.signi = value,
            "spell" => self.spell = value,
            "resona" => self.resona = value,
            "signi_craft" => self.signi_craft = value,
            "arts_craft" => self.arts_craft = value,
            "resona_craft" => self.resona_craft = value,
            "spell_craft" => self.spell_craft = value,
            "piece" => self.piece = value,
            "piece_relay" => self.piece_relay = value,
            "piece_craft" => self.piece_craft = value,
            "token" => self.token = value,
            "coin" => self.coin = value,
            _ => {} // 未知のコードは無視
        }
    }

    pub fn has_any(&self) -> bool {
        self.lrig
            || self.lrig_assist
            || self.arts
            || self.key
            || self.signi
            || self.spell
            || self.resona
            || self.signi_craft
            || self.arts_craft
            || self.resona_craft
            || self.spell_craft
            || self.piece
            || self.piece_relay
            || self.piece_craft
            || self.token
            || self.coin
    }

    pub fn get_selected_card_types(&self) -> Vec<CardType> {
        let mut selected = Vec::new();

        if self.lrig {
            selected.push(CardType::Lrig);
        }
        if self.lrig_assist {
            selected.push(CardType::LrigAssist);
        }
        if self.arts {
            selected.push(CardType::Arts);
        }
        if self.key {
            selected.push(CardType::Key);
        }
        if self.signi {
            selected.push(CardType::Signi);
        }
        if self.spell {
            selected.push(CardType::Spell);
        }
        if self.resona {
            selected.push(CardType::Resona);
        }
        if self.signi_craft {
            selected.push(CardType::SigniCraft);
        }
        if self.arts_craft {
            selected.push(CardType::ArtsCraft);
        }
        if self.resona_craft {
            selected.push(CardType::ResonaCraft);
        }
        if self.spell_craft {
            selected.push(CardType::SpellCraft);
        }
        if self.piece {
            selected.push(CardType::Piece);
        }
        if self.piece_relay {
            selected.push(CardType::PieceRelay);
        }
        if self.piece_craft {
            selected.push(CardType::PieceCraft);
        }
        if self.token {
            selected.push(CardType::Token);
        }
        if self.coin {
            selected.push(CardType::Coin);
        }

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

#[derive(Debug, Clone, PartialEq)]
pub struct PowerFilter {
    pub min_power: Option<i32>,
}

impl PowerFilter {
    pub fn new() -> Self {
        Self { min_power: None }
    }

    pub fn has_any(&self) -> bool {
        self.min_power.is_some()
    }

    pub fn set_threshold(&mut self, threshold: Option<i32>) {
        self.min_power = threshold;
    }

    pub fn threshold_options() -> Vec<i32> {
        vec![
            2000, 3000, 4000, 5000, 7000, 8000, 10000, 12000, 13000, 15000,
        ]
    }

    pub fn label_for_threshold(threshold: i32) -> String {
        format!("{}+", threshold)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlassFilter {
    pub selected_bits: u64,
}

impl KlassFilter {
    pub fn new() -> Self {
        Self { selected_bits: 0 }
    }

    pub fn has_any(&self) -> bool {
        self.selected_bits != 0
    }

    pub fn toggle_klass(&mut self, klass_id: i64) {
        use datapack::gen::klasses::get_klass_bit_position;

        if let Some(bit_position) = get_klass_bit_position(klass_id) {
            let bit_mask = 1u64 << bit_position;
            self.selected_bits ^= bit_mask;
        }
    }

    pub fn is_klass_selected(&self, klass_id: i64) -> bool {
        use datapack::gen::klasses::get_klass_bit_position;

        if let Some(bit_position) = get_klass_bit_position(klass_id) {
            let bit_mask = 1u64 << bit_position;
            (self.selected_bits & bit_mask) != 0
        } else {
            false
        }
    }

    pub fn clear_all(&mut self) {
        self.selected_bits = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LBFilter {
    pub selection: u8, // 0 = 指定なし, 1 = LBあり, 2 = LBなし
}

impl LBFilter {
    pub fn new() -> Self {
        Self { selection: 0 }
    }

    pub fn has_any(&self) -> bool {
        self.selection != 0
    }

    pub fn set_selection(&mut self, value: u8) {
        self.selection = value;
    }

    pub fn clear(&mut self) {
        self.selection = 0;
    }

    #[allow(unused)]
    pub fn get_label(&self) -> &'static str {
        match self.selection {
            1 => "LBあり",
            2 => "LBなし",
            _ => "指定なし",
        }
    }
}

// Klassマトリックス用のデータ構造
#[derive(Debug, Clone, PartialEq)]
pub struct KlassInfo {
    pub id: i64,
    pub cat1: String, // システム（奏像、奏武など）
    pub cat2: String, // タイプ（天使、ウェポンなど）
    pub cat3: String, // サブタイプ（ほとんど空）
    pub bit_position: u32,
}

impl KlassInfo {
    pub fn display_name(&self) -> String {
        if !self.cat2.is_empty() && !self.cat3.is_empty() {
            // cat2とcat3がある場合: "cat2/cat3"
            format!("{}/{}", self.cat2, self.cat3)
        } else if !self.cat2.is_empty() {
            // cat2がある場合: "cat2"
            self.cat2.clone()
        } else {
            // cat1のみの場合: "cat1" (解放派、闘争派、防衛派、奏元、精元)
            self.cat1.clone()
        }
    }
}

// Klassマトリックス構築用の関数
pub fn build_klass_matrix() -> Vec<(String, Vec<KlassInfo>)> {
    use datapack::gen::klasses::KLASS_LIST;
    use std::collections::HashMap;

    let mut result = Vec::new();

    // cat1のみのエントリ（解放派、闘争派、防衛派、奏元）を個別に処理
    let mut standalone_klasses = Vec::new();
    let mut cat1_systems: HashMap<String, Vec<KlassInfo>> = HashMap::new();

    for &(id, cat1, cat2, cat3, bit_position) in KLASS_LIST.iter() {
        // 精系クラス（精像、精武、精羅、精械、精生、精元）をUI上で非表示にする
        if cat1.starts_with("精") {
            continue;
        }

        let klass_info = KlassInfo {
            id,
            cat1: cat1.to_string(),
            cat2: cat2.to_string(),
            cat3: cat3.to_string(),
            bit_position,
        };

        if cat2.is_empty() {
            // cat1のみ（解放派、闘争派、防衛派、奏元）
            standalone_klasses.push(klass_info);
        } else {
            // cat1+cat2のシステム（奏像、奏武など）
            cat1_systems
                .entry(cat1.to_string())
                .or_default()
                .push(klass_info);
        }
    }

    // cat1+cat2システムを追加（奏系のみ）
    let mut sorted_systems: Vec<(String, Vec<KlassInfo>)> = cat1_systems.into_iter().collect();
    sorted_systems.sort_by(|a, b| {
        let order_a = get_system_order(&a.0);
        let order_b = get_system_order(&b.0);
        order_a.cmp(&order_b)
    });

    // 各システム内のタイプも並び替え
    for (_, types) in &mut sorted_systems {
        types.sort_by(|a, b| a.cat2.cmp(&b.cat2));
    }

    result.extend(sorted_systems);

    // standalone klassesを最後に追加（独立したカテゴリとして）
    if !standalone_klasses.is_empty() {
        standalone_klasses.sort_by(|a, b| {
            let order_a = get_system_order(&a.cat1);
            let order_b = get_system_order(&b.cat1);
            order_a.cmp(&order_b)
        });
        result.push(("その他".to_string(), standalone_klasses));
    }

    result
}

fn get_system_order(system: &str) -> usize {
    match system {
        "奏像" => 0,
        "奏武" => 1,
        "奏羅" => 2,
        "奏械" => 3,
        "奏生" => 4,
        "奏元" => 5,
        "精像" => 6,
        "精武" => 7,
        "精羅" => 8,
        "精械" => 9,
        "精生" => 10,
        "精元" => 11,
        "解放派" => 12,
        "闘争派" => 13,
        "防衛派" => 14,
        // 色単体
        "白" => 15,
        "赤" => 16,
        "青" => 17,
        "緑" => 18,
        "黒" => 19,
        _ => 999,
    }
}
