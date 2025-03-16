mod gen;

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
        &'static str,  // level
        &'static str, // limit
        &'static str, // limit_ex
        &'static str,  // power
        u8,           // has_burst
        &'static str, // skill_text
        &'static str, // burst_text
        u8,             // format
        &'static str, // story
        &'static str, // rarity
        &'static str, // url
        u8, // card_type
        u8, // product
        u8, // timing
        i64, // feature_bits1
        i64, // feature_bits2
    ),
);

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
            c.6,  // level
            c.7,    // limit
            c.8,    // limit_ex
            c.9,    // power
            c.10,    // has_burst
            c.11,    // skill_text
            c.12,    // burst_text
            c.13,    // format
            c.14,    // story
            c.15,    // rarity
            c.16,    // url
            c.17,    // card_type
            c.18,    // product
            c.19,    // timing
            c.20,    // feature_bits1
            c.21,    // feature_bits2
        )
    }
}

#[wasm_bindgen]
pub fn get_by_id(id: i32) -> String {
    let found = gen::cards::CARD_LIST.iter().find(|c| c.0 == id).unwrap();
    let cc = CardCompact(*found);
    cc.to_string()
}
