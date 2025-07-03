use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{RequestInit, RequestMode, Response};

use crate::utils::maintenance::{get_fixed_data_server_url, is_maintenance_mode};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FeatureOverride {
    pronunciation: String,
    features: Vec<String>,
    burst_features: Vec<String>,
    note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverrideResponse {
    pronunciation: String,
    features: Vec<String>,
    burst_features: Vec<String>,
    note: Option<String>,
}

// 実際のCardFeatureとBurstFeatureをdatapackから取得する関数
fn get_card_features_by_category() -> Vec<(String, Vec<String>)> {
    datapack::extract_card_features_grouped()
}

fn get_burst_features() -> Vec<String> {
    datapack::extract_burst_features_all()
}

#[component]
pub fn FeatureEditPage() -> impl IntoView {
    // Check if maintenance mode is enabled
    if !is_maintenance_mode() {
        return view! {
            <div class="container mx-auto px-4 py-8">
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                    <h2 class="text-xl font-bold">"機能編集モードは無効です"</h2>
                    <p>"この機能はローカル環境でのみ利用可能です。"</p>
                </div>
            </div>
        }
        .into_any();
    }

    let params = use_params_map();
    let pronunciation = move || params.read().get("pronunciation").unwrap_or_default();

    let (_current_override, set_current_override) = signal(None::<OverrideResponse>);
    let (selected_features, set_selected_features) = signal(Vec::<String>::new());
    let (selected_burst_features, set_selected_burst_features) = signal(Vec::<String>::new());
    let (note, set_note) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error_message, set_error_message) = signal(None::<String>);
    let (success_message, set_success_message) = signal(None::<String>);

    // Load on mount
    Effect::new(move |_| {
        let pronunciation = pronunciation();
        if pronunciation.is_empty() {
            return;
        }

        spawn_local(async move {
            let server_url = get_fixed_data_server_url();
            let url = format!("{server_url}/api/overrides/{pronunciation}");

            match load_override_from_api(&url).await {
                Ok(Some(override_data)) => {
                    // オーバーライドが存在する場合はそれを使用
                    set_selected_features.set(override_data.features.clone());
                    set_selected_burst_features.set(override_data.burst_features.clone());
                    set_note.set(override_data.note.clone().unwrap_or_default());
                    set_current_override.set(Some(override_data));
                }
                Ok(None) => {
                    // オーバーライドが存在しない場合は該当カードのフィーチャーで初期化
                    if let Some((card_features, burst_features)) =
                        datapack::get_card_features_by_pronunciation(&pronunciation)
                    {
                        set_selected_features.set(card_features);
                        set_selected_burst_features.set(burst_features);
                    }
                    set_current_override.set(None);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("読み込みエラー: {e}")));
                }
            }
        });
    });

    let toggle_feature = move |feature: String| {
        set_selected_features.update(|features| {
            if let Some(pos) = features.iter().position(|f| f == &feature) {
                features.remove(pos);
            } else {
                features.push(feature);
            }
        });
    };

    let toggle_burst_feature = move |feature: String| {
        set_selected_burst_features.update(|features| {
            if let Some(pos) = features.iter().position(|f| f == &feature) {
                features.remove(pos);
            } else {
                features.push(feature);
            }
        });
    };

    let save_override = move || {
        let pronunciation = pronunciation();
        if pronunciation.is_empty() {
            set_error_message.set(Some("読み方が指定されていません".to_string()));
            return;
        }

        set_loading.set(true);
        set_error_message.set(None);
        set_success_message.set(None);

        let override_data = FeatureOverride {
            pronunciation: pronunciation.clone(),
            features: selected_features.get(),
            burst_features: selected_burst_features.get(),
            note: if note.get().trim().is_empty() {
                None
            } else {
                Some(note.get())
            },
        };

        spawn_local(async move {
            let server_url = get_fixed_data_server_url();
            let url = format!("{server_url}/api/overrides");

            match save_override_to_api(&url, &override_data).await {
                Ok(_) => {
                    set_success_message.set(Some("保存しました".to_string()));
                    // Reload to get updated data
                    let reload_url = format!(
                        "{}/api/overrides/{}",
                        get_fixed_data_server_url(),
                        pronunciation
                    );
                    if let Ok(Some(updated_data)) = load_override_from_api(&reload_url).await {
                        set_current_override.set(Some(updated_data));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("保存エラー: {e}")));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container mx-auto px-4 py-8">
            <div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded mb-6">
                <h1 class="text-2xl font-bold">"フィーチャー編集モード"</h1>
                <p>"読み方: " <span class="font-mono">{pronunciation}</span></p>
            </div>

            {move || error_message.get().map(|msg| view! {
                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
                    {msg}
                </div>
            })}

            {move || success_message.get().map(|msg| view! {
                <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
                    {msg}
                </div>
            })}

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // CardFeatures
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-bold mb-4">"カードフィーチャー"</h2>
                    {
                        let card_features_by_category = get_card_features_by_category();
                        card_features_by_category.into_iter().map(|(category, features)| {
                            view! {
                                <div class="mb-6">
                                    <h3 class="text-lg font-semibold mb-2 text-gray-700">{category}</h3>
                                    <div class="grid grid-cols-2 md:grid-cols-3 gap-2">
                                        {features.into_iter().map(|feature| {
                                            let feature_name = feature.clone();
                                            let feature_name_for_is_selected = feature_name.clone();
                                            let is_selected = move || selected_features.get().contains(&feature_name_for_is_selected);
                                            let toggle_fn = {
                                                let feature_name = feature_name.clone();
                                                move |_| toggle_feature(feature_name.clone())
                                            };
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "px-3 py-2 text-sm rounded border transition-colors {}",
                                                        if is_selected() {
                                                            "bg-blue-500 text-white border-blue-500"
                                                        } else {
                                                            "bg-white text-gray-700 border-gray-300 hover:bg-gray-50"
                                                        }
                                                    )
                                                    on:click=toggle_fn
                                                >
                                                    {feature}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>

                // BurstFeatures
                <div class="bg-white rounded-lg shadow p-6">
                    <h2 class="text-xl font-bold mb-4">"ライフバーストフィーチャー"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                        {
                            let burst_features = get_burst_features();
                            burst_features.into_iter().map(|feature| {
                                let feature_name = feature.clone();
                                let feature_name_for_is_selected = feature_name.clone();
                                let is_selected = move || selected_burst_features.get().contains(&feature_name_for_is_selected);
                                let toggle_fn = {
                                    let feature_name = feature_name.clone();
                                    move |_| toggle_burst_feature(feature_name.clone())
                                };

                                view! {
                                    <button
                                        class=move || format!(
                                            "px-3 py-2 text-sm rounded border transition-colors {}",
                                            if is_selected() {
                                                "bg-green-500 text-white border-green-500"
                                            } else {
                                                "bg-white text-gray-700 border-gray-300 hover:bg-gray-50"
                                            }
                                        )
                                        on:click=toggle_fn
                                    >
                                        {feature}
                                    </button>
                                }
                            }).collect::<Vec<_>>()
                        }
                    </div>

                    <div class="mt-6">
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "メモ（修正理由など）"
                        </label>
                        <textarea
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                            rows="3"
                            placeholder="修正理由や特記事項を入力..."
                            prop:value=move || note.get()
                            on:input=move |ev| set_note.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="mt-6 flex space-x-3">
                        <button
                            class=move || format!(
                                "px-6 py-2 rounded font-medium transition-colors {}",
                                if loading.get() {
                                    "bg-gray-400 text-white cursor-not-allowed"
                                } else {
                                    "bg-blue-500 text-white hover:bg-blue-600"
                                }
                            )
                            disabled=move || loading.get()
                            on:click=move |_| save_override()
                        >
                            {move || if loading.get() { "保存中..." } else { "保存" }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

async fn load_override_from_api(url: &str) -> Result<Option<OverrideResponse>, String> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request =
        web_sys::Request::new_with_str_and_init(url, &opts).map_err(|_| "リクエスト作成エラー")?;

    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "ネットワークエラー")?;

    let resp: Response = resp_value.dyn_into().unwrap();

    if resp.status() == 404 {
        return Ok(None);
    }

    if !resp.ok() {
        return Err(format!("HTTPエラー: {}", resp.status()));
    }

    let text = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
        .await
        .map_err(|_| "レスポンス読み取りエラー")?;

    let override_data: OverrideResponse = serde_json::from_str(&text.as_string().unwrap())
        .map_err(|e| format!("JSONパースエラー: {e}"))?;

    Ok(Some(override_data))
}

async fn save_override_to_api(url: &str, data: &FeatureOverride) -> Result<(), String> {
    let json = serde_json::to_string(data).map_err(|e| format!("JSONシリアライズエラー: {e}"))?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&wasm_bindgen::JsValue::from_str(&json));

    let request =
        web_sys::Request::new_with_str_and_init(url, &opts).map_err(|_| "リクエスト作成エラー")?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|_| "ヘッダー設定エラー")?;

    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "ネットワークエラー")?;

    let resp: Response = resp_value.dyn_into().unwrap();

    if !resp.ok() {
        return Err(format!("HTTPエラー: {}", resp.status()));
    }

    Ok(())
}
