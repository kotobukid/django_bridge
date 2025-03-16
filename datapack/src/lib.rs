mod gen;

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
                           // i64, // feature_bits1
                           // i64, // feature_bits2
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
        }
    }
}

impl Display for CardCompact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = self.0;
        write!(
            f,
            "id: {}\n name: {}\n code: {}\n pronunciation: {}\n color: {}\n cost:{}\n level:{}\n limit:{}\n limit_ex:{}\n power:{}\n has_burst:{}\n skill_text:{}\n burst_text:{}\n format:{}\n story: {}\n rarity: {}\n url: {}\n card_type: {}\n product: {}\n timing: {}\n feature1: {}\n feature2: {}\n",
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
pub fn fetch_by_f_bits(bit1: i64, bits2: i64) -> JsValue {
    let cards: Vec<CardExport> = gen::cards::CARD_LIST
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
        .collect();
    serde_wasm_bindgen::to_value(&cards).unwrap()
}
