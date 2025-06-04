mod gen;

use color;
use feature::feature::export_features;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

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
#[derive(Serialize)]
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
pub fn fetch_by_f_bits(bit1: i64, bits2: i64) -> Vec<CardExport> {
    gen::cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;

            // 条件関数の確定
            if bit1 == 0 && bits2 == 0 {
                true
            } else if bits2 == 0 || bits2 == 1 {
                (feature_bits1 & bit1) != 0
            } else if bit1 == 0 || bit1 == 1 {
                (feature_bits2 & bits2) != 0
            } else {
                (feature_bits1 & bit1) == bit1 && (feature_bits2 & bits2) == bits2
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}

#[wasm_bindgen]
pub fn fetch_by_f_shifts(shift1: isize, shift2: isize) -> Vec<CardExport> {
    let bits1 = 1_i64 << shift1;
    let bits2 = 1_i64 << shift2;

    // web_sys::console::log_1(&format!("bits1: {}, bits2: {}", bits1, bits2).into());

    gen::cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;

            // 条件関数の確定
            if bits1 == 0 && bits2 == 0 {
                true
            } else if bits2 == 0 || bits2 == 1 {
                (feature_bits1 & bits1) != 0
            } else if bits1 == 0 || bits1 == 1 {
                (feature_bits2 & bits2) != 0
            } else {
                (feature_bits1 & bits1) == bits1 && (feature_bits2 & bits2) == bits2
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
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
pub fn fetch_by_features_and(features: &[i32]) -> Vec<CardExport> {
    // featuresは [shift1_1, shift2_1, shift1_2, shift2_2, ...] の形式
    // shift値が-1の場合は無視する
    gen::cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;
            
            // 全てのフィーチャーを満たすかチェック（AND条件）
            for i in (0..features.len()).step_by(2) {
                if i + 1 >= features.len() {
                    break;
                }
                
                let shift1 = features[i];
                let shift2 = features[i + 1];
                
                // 両方とも-1の場合はスキップ
                if shift1 < 0 && shift2 < 0 {
                    continue;
                }
                
                let bit1 = if shift1 >= 0 { 1_i64 << shift1 } else { 0 };
                let bit2 = if shift2 >= 0 { 1_i64 << shift2 } else { 0 };
                
                let has_feature = if bit1 > 0 && bit2 > 0 {
                    (feature_bits1 & bit1) != 0 && (feature_bits2 & bit2) != 0
                } else if bit1 > 0 {
                    (feature_bits1 & bit1) != 0
                } else if bit2 > 0 {
                    (feature_bits2 & bit2) != 0
                } else {
                    false
                };
                
                if !has_feature {
                    return false;
                }
            }
            
            true
        })
        .map(|c| CardExport::from(c))
        .collect()
}

#[wasm_bindgen]
pub fn fetch_by_combined_bits(bit1: i64, bit2: i64, mode: &str) -> Vec<CardExport> {
    gen::cards::CARD_LIST
        .iter()
        .filter(|c| {
            let feature_bits1 = c.20;
            let feature_bits2 = c.21;
            
            match mode {
                "and" => {
                    // AND条件: 指定されたビットが全て立っている
                    (bit1 == 0 || (feature_bits1 & bit1) == bit1) &&
                    (bit2 == 0 || (feature_bits2 & bit2) == bit2)
                },
                "or" => {
                    // OR条件: 指定されたビットのいずれかが立っている
                    if bit1 == 0 && bit2 == 0 {
                        true
                    } else {
                        (bit1 > 0 && (feature_bits1 & bit1) != 0) ||
                        (bit2 > 0 && (feature_bits2 & bit2) != 0)
                    }
                },
                _ => true
            }
        })
        .map(|c| CardExport::from(c))
        .collect()
}
