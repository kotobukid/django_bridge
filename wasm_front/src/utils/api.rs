use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{RequestInit, RequestMode, Response};

use super::maintenance::get_fixed_data_server_url;

#[derive(Debug, Clone, Deserialize)]
pub struct OverridePronunciations {
    pub pronunciations: Vec<String>,
}

pub async fn fetch_override_pronunciations() -> Result<Vec<String>, String> {
    let server_url = get_fixed_data_server_url();
    if server_url.is_empty() {
        // Not in maintenance mode
        return Ok(Vec::new());
    }

    let url = format!("{}/api/overrides/pronunciations", server_url);
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = web_sys::Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| "Failed to create request")?;

    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Network error")?;

    let resp: Response = resp_value.dyn_into().unwrap();
    
    if !resp.ok() {
        return Err(format!("HTTP error: {}", resp.status()));
    }

    let text = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
        .await
        .map_err(|_| "Failed to read response")?;

    let pronunciations: Vec<String> = serde_json::from_str(&text.as_string().unwrap())
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(pronunciations)
}