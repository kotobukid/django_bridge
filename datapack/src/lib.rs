mod extract;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}{name}!")
}

#[wasm_bindgen]
pub fn say_goodbye() -> String {
    "Goodbye!".to_string()
}
