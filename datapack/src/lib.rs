pub mod filter;
pub mod gen;

use color;
use feature::feature::export_features;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

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
            ex1: v.22.to_string(),
        }
    }
}

impl Display for CardCompact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = self.0;
        write!(
            f,
            "id: {}\n name: {}\n code: {}\n pronunciation: {}\n color: {}\n cost:{}\n level:{}\n limit:{}\n limit_ex:{}\n power:{}\n has_burst:{}\n skill_text:{}\n burst_text:{}\n format:{}\n story: {}\n rarity: {}\n url: {}\n card_type: {}\n product: {}\n timing: {}\n feature1: {}\n feature2: {}\n ex1: {}\n",
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
            c.22,   // ex1
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
            ex1: self.ex1.clone(),
        }
    }
}
