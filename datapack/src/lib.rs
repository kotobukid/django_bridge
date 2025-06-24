pub mod filter;
pub mod gen;
pub mod text_search;

use color::{self, Color};
use feature::feature::{export_features, export_burst_features, CardFeature, BurstFeature};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;
use gen::klasses::{KLASS_LIST, get_klass_display_name};
use gen::colors::COLOR_THEMES;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    Coin,
    Unknown,
}

impl CardType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => CardType::Lrig,           // ルリグ
            2 => CardType::Arts,           // アーツ  
            3 => CardType::LrigAssist,     // アシストルリグ
            4 => CardType::Piece,          // ピース
            5 => CardType::Signi,          // シグニ
            6 => CardType::Spell,          // スペル
            7 => CardType::Resona,         // レゾナ
            8 => CardType::Key,            // キー
            9 => CardType::ArtsCraft,      // クラフトアーツ
            10 => CardType::SigniCraft,    // クラフトシグニ
            11 => CardType::SpellCraft,    // クラフトスペル
            12 => CardType::PieceRelay,    // リレーピース
            13 => CardType::PieceCraft,    // クラフトピース
            14 => CardType::ResonaCraft,   // クラフトレゾナ
            15 => CardType::Token,         // トークン
            16 => CardType::Coin,          // コイン
            _ => CardType::Unknown,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CardType::Lrig => 1,           // ルリグ
            CardType::Arts => 2,           // アーツ  
            CardType::LrigAssist => 3,     // アシストルリグ
            CardType::Piece => 4,          // ピース
            CardType::Signi => 5,          // シグニ
            CardType::Spell => 6,          // スペル
            CardType::Resona => 7,         // レゾナ
            CardType::Key => 8,            // キー
            CardType::ArtsCraft => 9,      // クラフトアーツ
            CardType::SigniCraft => 10,    // クラフトシグニ
            CardType::SpellCraft => 11,    // クラフトスペル
            CardType::PieceRelay => 12,    // リレーピース
            CardType::PieceCraft => 13,    // クラフトピース
            CardType::ResonaCraft => 14,   // クラフトレゾナ
            CardType::Token => 15,         // トークン
            CardType::Coin => 16,          // コイン
            CardType::Unknown => 0,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            CardType::Lrig => "ルリグ",
            CardType::LrigAssist => "ルリグアシスト",
            CardType::Arts => "アーツ",
            CardType::Key => "キー",
            CardType::Signi => "シグニ",
            CardType::Spell => "スペル",
            CardType::Resona => "レゾナ",
            CardType::SigniCraft => "シグニクラフト",
            CardType::ArtsCraft => "アーツクラフト",
            CardType::ResonaCraft => "レゾナクラフト",
            CardType::SpellCraft => "スペルクラフト",
            CardType::Piece => "ピース",
            CardType::PieceRelay => "ピースリレー",
            CardType::PieceCraft => "ピースクラフト",
            CardType::Token => "トークン",
            CardType::Coin => "コイン",
            CardType::Unknown => "不明",
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}{name}!")
}

#[wasm_bindgen]
pub fn say_goodbye() -> String {
    "Goodbye!".to_string()
}

struct CardCompact(
    (
        i32,          // id
        &'static str, // name
        &'static str, // code
        &'static str, // pronunciation
        u32,          // color
        &'static str, // cost
        &'static str, // level
        &'static str, // limit
        &'static str, // limit_ex
        &'static str, // power
        u8,           // has_burst
        &'static str, // skill_text
        &'static str, // burst_text
        u8,           // format
        &'static str, // story
        &'static str, // rarity
        &'static str, // url
        u8,           // card_type
        u8,           // product
        u8,           // timing
        i64,          // feature_bits1
        i64,          // feature_bits2
        u64,          // klass_bits
        i64,          // burst_bits
        &'static str, // ex1
    ),
);

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq)]
pub struct CardExport {
    id: i32,               // id
    name: String,          // name
    code: String,          // code
    pronunciation: String, // pronunciation
    color: u32,            // color
    cost: String,          // cost
    level: String,         // level
    limit: String,         // limit
    limit_ex: String,      // limit_ex
    power: String,         // power
    has_burst: u8,         // has_burst
    skill_text: String,    // skill_text
    burst_text: String,    // burst_text
    format: u8,            // format
    story: String,         // story
    rarity: String,        // rarity
    url: String,           // url
    card_type: u8,         // card_type
    product: u8,           // product
    timing: u8,            // timing
    feature_bits1: i64,    // feature_bits1
    feature_bits2: i64,    // feature_bits2
    klass_bits: u64,       // klass_bits
    burst_bits: i64,       // burst_bits
    ex1: String,
}

#[wasm_bindgen]
impl CardExport {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn pronunciation(&self) -> String {
        self.pronunciation.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> u32 {
        self.color
    }

    #[wasm_bindgen(getter)]
    pub fn cost(&self) -> String {
        self.cost.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> String {
        self.level.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn limit(&self) -> String {
        self.limit.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn limit_ex(&self) -> String {
        self.limit_ex.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn power(&self) -> String {
        self.power.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn has_burst(&self) -> u8 {
        self.has_burst
    }

    #[wasm_bindgen(getter)]
    pub fn skill_text(&self) -> String {
        self.skill_text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn burst_text(&self) -> String {
        self.burst_text.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn format(&self) -> u8 {
        self.format
    }

    #[wasm_bindgen(getter)]
    pub fn story(&self) -> String {
        self.story.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn rarity(&self) -> String {
        self.rarity.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn card_type(&self) -> u8 {
        self.card_type
    }

    #[wasm_bindgen(getter)]
    pub fn product(&self) -> u8 {
        self.product
    }

    #[wasm_bindgen(getter)]
    pub fn timing(&self) -> u8 {
        self.timing
    }

    #[wasm_bindgen(getter)]
    pub fn feature_bits1(&self) -> i64 {
        self.feature_bits1
    }

    #[wasm_bindgen(getter)]
    pub fn feature_bits2(&self) -> i64 {
        self.feature_bits2
    }

    #[wasm_bindgen(getter)]
    pub fn klass_bits(&self) -> u64 {
        self.klass_bits
    }

    #[wasm_bindgen(getter)]
    pub fn burst_bits(&self) -> i64 {
        self.burst_bits
    }

    #[wasm_bindgen(getter)]
    pub fn ex1(&self) -> String {
        self.ex1.clone()
    }
}

impl
    From<&(
        i32,          // id
        &'static str, // name
        &'static str, // code
        &'static str, // pronunciation
        u32,          // color
        &'static str, // cost
        &'static str, // level
        &'static str, // limit
        &'static str, // limit_ex
        &'static str, // power
        u8,           // has_burst
        &'static str, // skill_text
        &'static str, // burst_text
        u8,           // format
        &'static str, // story
        &'static str, // rarity
        &'static str, // url
        u8,           // card_type
        u8,           // product
        u8,           // timing
        i64,          // feature_bits1
        i64,          // feature_bits2
        u64,          // klass_bits
        i64,          // burst_bits
        &'static str, // ex1
    )> for CardExport
{
    fn from(
        v: &(
            i32,          // id
            &'static str, // name
            &'static str, // code
            &'static str, // pronunciation
            u32,          // color
            &'static str, // cost
            &'static str, // level
            &'static str, // limit
            &'static str, // limit_ex
            &'static str, // power
            u8,           // has_burst
            &'static str, // skill_text
            &'static str, // burst_text
            u8,           // format
            &'static str, // story
            &'static str, // rarity
            &'static str, // url
            u8,           // card_type
            u8,           // product
            u8,           // timing
            i64,          // feature_bits1
            i64,          // feature_bits2
            u64,          // klass_bits
            i64,          // burst_bits
            &'static str, // ex1
        ),
    ) -> Self {
        // let v = v.0;
        CardExport {
            id: v.0,
            name: v.1.to_string(),
            code: v.2.to_string(),
            pronunciation: v.3.to_string(),
            color: v.4,
            cost: v.5.to_string(),
            level: v.6.to_string(),
            limit: v.7.to_string(),
            limit_ex: v.8.to_string(),
            power: v.9.to_string(),
            has_burst: v.10,
            skill_text: v.11.to_string(),
            burst_text: v.12.to_string(),
            format: v.13,
            story: v.14.to_string(),
            rarity: v.15.to_string(),
            url: v.16.to_string(),
            card_type: v.17,
            product: v.18,
            timing: v.19,
            feature_bits1: v.20,
            feature_bits2: v.21,
            klass_bits: v.22,
            burst_bits: v.23,
            ex1: v.24.to_string(),
        }
    }
}

impl Display for CardCompact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = self.0;
        write!(
            f,
            "id: {}\n name: {}\n code: {}\n pronunciation: {}\n color: {}\n cost:{}\n level:{}\n limit:{}\n limit_ex:{}\n power:{}\n has_burst:{}\n skill_text:{}\n burst_text:{}\n format:{}\n story: {}\n rarity: {}\n url: {}\n card_type: {}\n product: {}\n timing: {}\n feature1: {}\n feature2: {}\n klass_bits: {}\n burst_bits: {}\n ex1: {}\n",
            c.0,    // id
            c.1,    // name
            c.2,    // code
            c.3,    // pron
            c.4,    // color
            c.5,    // cost
            c.6,    // level
            c.7,    // limit
            c.8,    // limit_ex
            c.9,    // power
            c.10,   // has_burst
            c.11,   // skill_text
            c.12,   // burst_text
            c.13,   // format
            c.14,   // story
            c.15,   // rarity
            c.16,   // url
            c.17,   // card_type
            c.18,   // product
            c.19,   // timing
            c.20,   // feature_bits1
            c.21,   // feature_bits2
            c.22,   // klass_bits
            c.23,   // burst_bits
            c.24,   // ex1
        )
    }
}

#[wasm_bindgen]
pub fn get_by_id(id: i32) -> String {
    let found = gen::cards::CARD_LIST.iter().find(|c| c.0 == id).unwrap();
    let cc = CardCompact(*found);
    cc.to_string()
}

#[wasm_bindgen]
pub fn fetch_by_f_bits(bit1: i64, bits2: i64) -> String {
    let cards = filter::filter_by_f_bits(bit1, bits2);
    format!("Found {} cards", cards.len())
}

#[wasm_bindgen]
pub fn fetch_by_f_shifts(shift1: isize, shift2: isize) -> String {
    let cards = filter::filter_by_f_shifts(shift1, shift2);
    format!("Found {} cards", cards.len())
}

#[wasm_bindgen]
pub fn feature_conditions() -> JsValue {
    let data = export_features();
    serde_wasm_bindgen::to_value(&data).unwrap()
}

#[wasm_bindgen]
pub fn bits_to_gradient(bits: i32) -> JsValue {
    let style = color::Colors::bits_to_gradient(bits);
    serde_wasm_bindgen::to_value(&style).unwrap()
}

#[wasm_bindgen]
pub fn fetch_by_features_and(features: &[i32]) -> String {
    let cards = filter::filter_by_features_and(features);
    format!("Found {} cards with features {:?}", cards.len(), features)
}

#[wasm_bindgen]
pub fn fetch_by_combined_bits_and(bit1: i64, bit2: i64) -> String {
    let cards = filter::filter_by_combined_bits(bit1, bit2, "and");
    format!(
        "Found {} cards (AND: bit1={}, bit2={})",
        cards.len(),
        bit1,
        bit2
    )
}

#[wasm_bindgen]
pub fn fetch_by_combined_bits_or(bit1: i64, bit2: i64) -> String {
    let cards = filter::filter_by_combined_bits(bit1, bit2, "or");
    format!(
        "Found {} cards (OR: bit1={}, bit2={})",
        cards.len(),
        bit1,
        bit2
    )
}

// Native Rust functions for Leptos (not exposed to WASM)
pub fn get_all_cards() -> Result<Vec<CardExport>, String> {
    Ok(gen::cards::CARD_LIST
        .iter()
        .map(|c| CardExport::from(c))
        .collect())
}

pub fn fetch_by_colors(cards: &[CardExport], color_bits: u32) -> Vec<CardExport> {
    cards
        .iter()
        .filter(|c| (c.color & color_bits) != 0)
        .cloned()
        .collect()
}

pub fn fetch_by_colors_and(cards: &[CardExport], color_bits: u32) -> Vec<CardExport> {
    cards
        .iter()
        .filter(|c| (c.color & color_bits) == color_bits)
        .cloned()
        .collect()
}

pub fn bits_to_gradient_native(bits: i32) -> String {
    color::Colors::bits_to_gradient(bits)
}

pub fn fetch_by_features_and_native(
    cards: &[CardExport],
    feature_names: &[String],
) -> Vec<CardExport> {
    use feature::feature::HashSetToBits;
    use std::collections::HashSet;

    // feature名からCardFeatureに変換してビットを計算
    let mut feature_set = HashSet::new();

    for name in feature_names {
        if let Ok(feature) = parse_feature_name(name) {
            feature_set.insert(feature);
        }
    }

    if feature_set.is_empty() {
        return cards.to_vec();
    }

    let (bits1, bits2) = feature_set.to_bits();

    cards
        .iter()
        .filter(|c| {
            (bits1 == 0 || (c.feature_bits1 & bits1) == bits1)
                && (bits2 == 0 || (c.feature_bits2 & bits2) == bits2)
        })
        .cloned()
        .collect()
}

// feature名からCardFeatureに変換する関数
pub fn parse_feature_name(name: &str) -> Result<feature::feature::CardFeature, String> {
    feature::labels::FEATURE_LABELS
        .get(name)
        .cloned()
        .ok_or_else(|| format!("Unknown feature: {}", name))
}

// 色とfeatureの複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_and_features_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    filtered_cards
}

// 色、feature、カード種別の複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_and_card_types_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // 最後にカード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    filtered_cards
}

// 色、feature、カード種別、商品の複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_card_types_and_products_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 最後に商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    filtered_cards
}

// 色、feature、カード種別、商品、レベルの複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_card_types_products_and_levels_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // 最後にレベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    filtered_cards
}


// Japanese text normalization utilities
fn normalize_japanese_text(text: &str) -> String {
    // Convert Hiragana to Katakana for consistent searching
    // This is a simplified version - in production you might want to use a proper Japanese text processing library
    let mut result = String::new();
    
    for c in text.chars() {
        if c >= 'ひ' && c <= 'ゔ' {
            // Convert Hiragana to Katakana (approximate conversion)
            let katakana_code = c as u32 + 0x60;
            if let Some(katakana_char) = char::from_u32(katakana_code) {
                result.push(katakana_char);
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result.to_lowercase()
}

fn text_matches(text: &str, search_term: &str) -> bool {
    if search_term.is_empty() {
        return true;
    }
    
    let normalized_text = normalize_japanese_text(text);
    let normalized_search = normalize_japanese_text(search_term);
    
    // Support both exact and partial matching
    normalized_text.contains(&normalized_search)
}

// Helper function to extract card features from bit flags
pub fn extract_card_features_from_bits(feature_bits1: i64, feature_bits2: i64) -> Vec<(String, String)> {
    let mut features = Vec::new();
    
    // Get all CardFeatures and check which ones are set
    let all_features = CardFeature::create_vec();
    
    for feature in all_features {
        let (bit_pos1, bit_pos2) = feature.to_bit_shifts();
        
        let has_feature = if bit_pos2 == 0 {
            (feature_bits1 & (1_i64 << bit_pos1)) != 0
        } else {
            (feature_bits2 & (1_i64 << bit_pos2)) != 0
        };
        
        if has_feature {
            let feature_label = feature.to_string();
            let tag = feature.tag();
            let tag_label = tag.to_string();
            
            // Remove the sort prefix (first 2 characters) from tag label
            let display_tag_label = if tag_label.len() > 2 {
                tag_label[2..].to_string()
            } else {
                tag_label
            };
            
            features.push((display_tag_label, feature_label));
        }
    }
    
    features
}

#[wasm_bindgen]
pub fn get_card_features_from_bits(feature_bits1: i64, feature_bits2: i64) -> JsValue {
    let features = extract_card_features_from_bits(feature_bits1, feature_bits2);
    serde_wasm_bindgen::to_value(&features).unwrap()
}

// Helper function to extract klass names from bit flags
pub fn extract_klass_names_from_bits(klass_bits: u64) -> Vec<String> {
    let mut klass_names = Vec::new();
    
    for klass_static in KLASS_LIST.iter() {
        let (klass_id, cat1, _, _, bit_position) = *klass_static;
        
        // 精系クラス（精像、精武、精羅、精械、精生、精元）をUI上で非表示にする
        if cat1.starts_with("精") {
            continue;
        }
        
        // Check if this klass bit is set
        if (klass_bits & (1u64 << bit_position)) != 0 {
            if let Some(display_name) = get_klass_display_name(klass_id) {
                klass_names.push(display_name);
            }
        }
    }
    
    klass_names
}

#[wasm_bindgen]
pub fn get_klass_names_from_bits(klass_bits: u64) -> JsValue {
    let klass_names = extract_klass_names_from_bits(klass_bits);
    serde_wasm_bindgen::to_value(&klass_names).unwrap()
}

// Helper function to extract burst features from bit flags
pub fn extract_burst_features_from_bits(burst_bits: i64) -> Vec<(String, String)> {
    let mut features = Vec::new();
    
    // Get all BurstFeatures and check which ones are set
    let all_burst_features = BurstFeature::create_vec();
    
    for feature in all_burst_features {
        let bit_pos = feature.to_bit_shift();
        
        let has_feature = (burst_bits & (1_i64 << bit_pos)) != 0;
        
        if has_feature {
            let feature_label = feature.to_string();
            let tag = feature.tag();
            let tag_label = tag.to_string();
            
            // Remove the sort prefix (first 2 characters) from tag label  
            let display_tag_label = if tag_label.len() > 2 {
                tag_label[2..].to_string()
            } else {
                tag_label
            };
            
            features.push((display_tag_label, feature_label));
        }
    }
    
    features
}

#[wasm_bindgen]
pub fn get_burst_features_from_bits(burst_bits: i64) -> JsValue {
    let features = extract_burst_features_from_bits(burst_bits);
    serde_wasm_bindgen::to_value(&features).unwrap()
}

// カラーテーマを取得する関数
pub fn get_color_theme_native(color_name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    COLOR_THEMES
        .iter()
        .find(|(name, _, _, _)| *name == color_name)
        .map(|(_, base, accent, light)| (*base, *accent, *light))
}

#[wasm_bindgen]
pub fn get_color_theme(color_name: &str) -> JsValue {
    let theme = get_color_theme_native(color_name);
    serde_wasm_bindgen::to_value(&theme).unwrap()
}

// カードの色ビットから主要色名を取得する関数
pub fn get_primary_color_name_from_bits(color_bits: u32) -> String {
    let colors = color::from_bits(color_bits as i32);
    if colors.is_empty() {
        return "Unknown".to_string();
    }
    
    // 最初の色を主要色として扱う
    match colors[0] {
        Color::White => "White".to_string(),
        Color::Blue => "Blue".to_string(),
        Color::Red => "Red".to_string(),
        Color::Black => "Black".to_string(),
        Color::Green => "Green".to_string(),
        Color::Colorless => "Colorless".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[wasm_bindgen]
pub fn get_primary_color_name(color_bits: u32) -> String {
    get_primary_color_name_from_bits(color_bits)
}

// Text search function for card name, code, and pronunciation
pub fn search_cards_by_text_native(
    cards: &[CardExport],
    search_term: &str,
) -> Vec<CardExport> {
    if search_term.trim().is_empty() {
        return cards.to_vec();
    }
    
    cards
        .iter()
        .filter(|card| {
            text_matches(&card.name, search_term) ||
            text_matches(&card.code, search_term) ||
            text_matches(&card.pronunciation, search_term)
        })
        .cloned()
        .collect()
}

// 色、feature、カード種別、商品、レベル、テキスト検索の複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_card_types_products_levels_and_text_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    search_text: &str,
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // レベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    // 最後にテキスト検索でフィルタリング（最適化バージョン使用）
    if !search_text.trim().is_empty() {
        filtered_cards = text_search::search_cards_by_text_optimized(&filtered_cards, search_text);
    }

    filtered_cards
}

// 色、feature、カード種別、商品、レベル、パワー閾値、テキスト検索の複合フィルタリング関数（全てAND条件）
// Klass対応版の複合フィルタリング関数
pub fn fetch_by_colors_features_card_types_products_levels_power_threshold_klass_and_text_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    min_power: Option<i32>,
    klass_bits: u64,
    search_text: &str,
) -> Vec<CardExport> {
    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // レベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    // パワー閾値でフィルタリング
    if let Some(threshold) = min_power {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| {
                if let Ok(power) = card.power.parse::<i32>() {
                    power >= threshold
                } else {
                    false
                }
            })
            .collect();
    }

    // Klassでフィルタリング（OR条件 - 選択されたKlassのいずれかに該当）
    if klass_bits != 0 {
        use crate::gen::klasses::has_klass_bits;
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| has_klass_bits(card.klass_bits, klass_bits))
            .collect();
    }

    // テキスト検索でフィルタリング
    if !search_text.is_empty() {
        let search_lower = search_text.to_lowercase();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| {
                card.name.to_lowercase().contains(&search_lower)
                    || card.skill_text.to_lowercase().contains(&search_lower)
                    || card.burst_text.to_lowercase().contains(&search_lower)
            })
            .collect();
    }

    filtered_cards
}

// 後方互換性のための旧関数（新しい関数に転送）
pub fn fetch_by_colors_features_card_types_products_levels_power_threshold_and_text_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    min_power: Option<i32>,
    search_text: &str,
) -> Vec<CardExport> {
    fetch_by_colors_features_card_types_products_levels_power_threshold_klass_and_text_native(
        cards,
        color_bits,
        feature_names,
        card_types,
        products,
        levels,
        min_power,
        0, // klass_bits = 0 (フィルタなし)
        search_text,
    )
}


// 色、feature、カード種別、商品、レベル、パワー、テキスト検索の複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_card_types_products_levels_powers_and_text_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    powers: &[String],
    search_text: &str,
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // レベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    // パワーでフィルタリング（OR条件）
    if !powers.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| powers.contains(&card.power))
            .collect();
    }

    // 最後にテキスト検索でフィルタリング（最適化バージョン使用）
    if !search_text.trim().is_empty() {
        filtered_cards = text_search::search_cards_by_text_optimized(&filtered_cards, search_text);
    }

    filtered_cards
}

// パワー範囲でフィルタリングする関数
pub fn filter_by_power_range_native(
    cards: &[CardExport],
    min_power: Option<i32>,
    max_power: Option<i32>,
    include_infinity: bool,
) -> Vec<CardExport> {
    cards
        .iter()
        .filter(|card| {
            // 空の文字列や"-"の場合はスキップ
            if card.power.is_empty() || card.power == "-" {
                return false;
            }
            
            // 無限大のケース
            if card.power == "∞" {
                return include_infinity;
            }
            
            // 数値として解析
            if let Ok(power_value) = card.power.parse::<i32>() {
                let above_min = min_power.map_or(true, |min| power_value >= min);
                let below_max = max_power.map_or(true, |max| power_value <= max);
                above_min && below_max
            } else {
                false
            }
        })
        .cloned()
        .collect()
}

// 完全なフィルタリング関数（パワー範囲フィルタリング付き）
pub fn fetch_by_all_filters_with_power_range_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    powers: &[String],
    min_power: Option<i32>,
    max_power: Option<i32>,
    include_infinity: bool,
    search_text: &str,
) -> Vec<CardExport> {

    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // レベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    // パワーの特定値でフィルタリング（OR条件）
    if !powers.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| powers.contains(&card.power))
            .collect();
    }

    // パワーの範囲でフィルタリング
    if min_power.is_some() || max_power.is_some() {
        filtered_cards = filter_by_power_range_native(&filtered_cards, min_power, max_power, include_infinity);
    }

    // 最後にテキスト検索でフィルタリング（最適化バージョン使用）
    if !search_text.trim().is_empty() {
        filtered_cards = text_search::search_cards_by_text_optimized(&filtered_cards, search_text);
    }

    filtered_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_japanese_text() {
        // Test basic functionality
        assert_eq!(normalize_japanese_text("hello"), "hello");
        
        // Test case conversion
        assert_eq!(normalize_japanese_text("Hello World"), "hello world");
        
        // Test that the function doesn't crash with Japanese text
        let japanese_text = "こんにちは";
        let result = normalize_japanese_text(japanese_text);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_text_matches() {
        // Basic matching
        assert!(text_matches("Hello World", "hello"));
        assert!(text_matches("Test Card", "card"));
        assert!(text_matches("WX24-001", "wx24"));
        
        // Empty search should match anything
        assert!(text_matches("anything", ""));
        
        // Non-matching
        assert!(!text_matches("hello", "world"));
    }

    #[test]
    fn test_search_cards_by_text_empty_search() {
        let cards = vec![
            CardExport {
                id: 1,
                name: "Test Card".to_string(),
                code: "WX24-001".to_string(),
                pronunciation: "テストカード".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                power: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            }
        ];
        
        // Empty search should return all cards
        let result = search_cards_by_text_native(&cards, "");
        assert_eq!(result.len(), 1);
        
        // Matching search
        let result = search_cards_by_text_native(&cards, "test");
        assert_eq!(result.len(), 1);
        
        // Non-matching search
        let result = search_cards_by_text_native(&cards, "nonexistent");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_filter_by_power_range() {
        let cards = vec![
            CardExport {
                id: 1,
                name: "Low Power".to_string(),
                power: "1000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 2,
                name: "Mid Power".to_string(),
                power: "5000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 3,
                name: "High Power".to_string(),
                power: "10000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 4,
                name: "Infinity Power".to_string(),
                power: "∞".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 5,
                name: "No Power".to_string(),
                power: "-".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
        ];
        
        // Test range filtering
        let result = filter_by_power_range_native(&cards, Some(3000), Some(8000), false);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 2);
        
        // Test minimum only
        let result = filter_by_power_range_native(&cards, Some(5000), None, false);
        assert_eq!(result.len(), 2);
        
        // Test maximum only
        let result = filter_by_power_range_native(&cards, None, Some(5000), false);
        assert_eq!(result.len(), 2);
        
        // Test including infinity
        let result = filter_by_power_range_native(&cards, Some(8000), None, true);
        assert_eq!(result.len(), 2); // 10000 and ∞
        
        // Test excluding infinity
        let result = filter_by_power_range_native(&cards, Some(8000), None, false);
        assert_eq!(result.len(), 1); // only 10000
    }

    #[test]
    fn test_fetch_by_powers() {
        let cards = vec![
            CardExport {
                id: 1,
                name: "Card 1".to_string(),
                power: "3000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 2,
                name: "Card 2".to_string(),
                power: "5000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
            CardExport {
                id: 3,
                name: "Card 3".to_string(),
                power: "10000".to_string(),
                code: "".to_string(),
                pronunciation: "".to_string(),
                color: 0,
                cost: "".to_string(),
                level: "".to_string(),
                limit: "".to_string(),
                limit_ex: "".to_string(),
                has_burst: 0,
                skill_text: "".to_string(),
                burst_text: "".to_string(),
                format: 0,
                story: "".to_string(),
                rarity: "".to_string(),
                url: "".to_string(),
                card_type: 0,
                product: 0,
                timing: 0,
                feature_bits1: 0,
                feature_bits2: 0,
                klass_bits: 0,
                burst_bits: 0,
                ex1: "".to_string(),
            },
        ];
        
        // Test specific power filtering (OR condition)
        let powers = vec!["3000".to_string(), "10000".to_string()];
        let result = fetch_by_colors_features_card_types_products_levels_powers_and_text_native(
            &cards,
            0,
            &[],
            &[],
            &[],
            &[],
            &powers,
            "",
        );
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|c| c.id == 1));
        assert!(result.iter().any(|c| c.id == 3));
    }
}

impl Clone for CardExport {
    fn clone(&self) -> Self {
        CardExport {
            id: self.id,
            name: self.name.clone(),
            code: self.code.clone(),
            pronunciation: self.pronunciation.clone(),
            color: self.color,
            cost: self.cost.clone(),
            level: self.level.clone(),
            limit: self.limit.clone(),
            limit_ex: self.limit_ex.clone(),
            power: self.power.clone(),
            has_burst: self.has_burst,
            skill_text: self.skill_text.clone(),
            burst_text: self.burst_text.clone(),
            format: self.format,
            story: self.story.clone(),
            rarity: self.rarity.clone(),
            url: self.url.clone(),
            card_type: self.card_type,
            product: self.product,
            timing: self.timing,
            feature_bits1: self.feature_bits1,
            feature_bits2: self.feature_bits2,
            klass_bits: self.klass_bits,
            burst_bits: self.burst_bits,
            ex1: self.ex1.clone(),
        }
    }
}

// BurstFeature条件を取得する関数（WASMバインディング用）
#[wasm_bindgen]
pub fn burst_feature_conditions() -> JsValue {
    let features = export_burst_features();
    serde_wasm_bindgen::to_value(&features).unwrap()
}

// BurstFeature名から対応するBurstFeatureを取得
fn get_burst_feature_by_name(name: &str) -> Option<BurstFeature> {
    feature::labels::BURST_FEATURE_LABELS.get(name).cloned()
}

// BurstFeature名のリストをビットに変換
fn convert_burst_feature_names_to_bits(feature_names: &[String]) -> i64 {
    let mut bits = 0_i64;
    for name in feature_names {
        if let Some(feature) = get_burst_feature_by_name(name) {
            let shift = feature.to_bit_shift();
            bits |= 1_i64 << shift;
        }
    }
    bits
}

// BurstFeatureでフィルタリングする関数
#[wasm_bindgen]
pub fn fetch_by_burst_features_and(feature_names: JsValue) -> Vec<CardExport> {
    let names: Vec<String> = serde_wasm_bindgen::from_value(feature_names).unwrap_or_default();
    let burst_bits = convert_burst_feature_names_to_bits(&names);
    filter::filter_by_burst_bits(burst_bits, "and")
}

// BurstFeatureでフィルタリングする関数（OR条件）
#[wasm_bindgen]
pub fn fetch_by_burst_features_or(feature_names: JsValue) -> Vec<CardExport> {
    let names: Vec<String> = serde_wasm_bindgen::from_value(feature_names).unwrap_or_default();
    let burst_bits = convert_burst_feature_names_to_bits(&names);
    filter::filter_by_burst_bits(burst_bits, "or")
}

// BurstFeature名でネイティブフィルタリング
pub fn fetch_by_burst_features_and_native(cards: &[CardExport], feature_names: &[String]) -> Vec<CardExport> {
    let burst_bits = convert_burst_feature_names_to_bits(feature_names);
    
    cards
        .iter()
        .filter(|card| {
            // AND条件: 指定されたビットが全て立っている
            burst_bits == 0 || (card.burst_bits & burst_bits) == burst_bits
        })
        .cloned()
        .collect()
}

// has_burstでフィルタリングする関数（ネイティブ版）
/// has_burst_value: 0 = 指定なし（全て表示）, 1 = LBあり, 2 = LBなし
pub fn filter_by_has_burst_native(cards: &[CardExport], has_burst_value: u8) -> Vec<CardExport> {
    match has_burst_value {
        0 => cards.to_vec(), // 指定なし = 全て表示
        1 => cards
            .iter()
            .filter(|card| card.has_burst == 1) // LBあり
            .cloned()
            .collect(),
        2 => cards
            .iter()
            .filter(|card| card.has_burst == 2) // LBなし
            .cloned()
            .collect(),
        _ => cards.to_vec(),
    }
}

// 色、feature、カード種別、商品、レベル、パワー閾値、Klass、has_burst、テキスト検索の複合フィルタリング関数（全てAND条件）
pub fn fetch_by_colors_features_card_types_products_levels_power_threshold_klass_has_burst_and_text_native(
    cards: &[CardExport],
    color_bits: u32,
    feature_names: &[String],
    card_types: &[CardType],
    products: &[u8],
    levels: &[String],
    min_power: Option<i32>,
    klass_bits: u64,
    has_burst: u8,
    search_text: &str,
) -> Vec<CardExport> {
    // まず色でフィルタリング（color_bits が 0 の場合はフィルタしない）
    let mut filtered_cards = if color_bits == 0 {
        cards.to_vec()
    } else {
        fetch_by_colors_and(cards, color_bits)
    };

    // 次にfeatureでフィルタリング
    if !feature_names.is_empty() {
        filtered_cards = fetch_by_features_and_native(&filtered_cards, feature_names);
    }

    // カード種別でフィルタリング
    if !card_types.is_empty() {
        let card_type_u8s: Vec<u8> = card_types.iter().map(|ct| ct.to_u8()).collect();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| card_type_u8s.contains(&card.card_type))
            .collect();
    }

    // 商品でフィルタリング
    if !products.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| products.contains(&card.product))
            .collect();
    }

    // レベルでフィルタリング（OR条件）
    if !levels.is_empty() {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| levels.contains(&card.level))
            .collect();
    }

    // パワー閾値でフィルタリング
    if let Some(threshold) = min_power {
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| {
                if let Ok(power) = card.power.parse::<i32>() {
                    power >= threshold
                } else {
                    false
                }
            })
            .collect();
    }

    // Klassでフィルタリング（OR条件 - 選択されたKlassのいずれかに該当）
    if klass_bits != 0 {
        use crate::gen::klasses::has_klass_bits;
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| has_klass_bits(card.klass_bits, klass_bits))
            .collect();
    }

    // has_burstでフィルタリング
    if has_burst != 0 {
        filtered_cards = filter_by_has_burst_native(&filtered_cards, has_burst);
    }

    // テキスト検索でフィルタリング
    if !search_text.is_empty() {
        let search_lower = search_text.to_lowercase();
        filtered_cards = filtered_cards
            .into_iter()
            .filter(|card| {
                card.name.to_lowercase().contains(&search_lower)
                    || card.skill_text.to_lowercase().contains(&search_lower)
                    || card.burst_text.to_lowercase().contains(&search_lower)
            })
            .collect();
    }

    filtered_cards
}
