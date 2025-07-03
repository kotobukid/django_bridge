use crate::components::SvgToggleSwitch;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct FeatureTag {
    id: String,
    display_name: String,
}

#[derive(Debug, Clone)]
struct CardFeature {
    name: String,
}

// 実際のfeatureモジュールからデータを取得する関数
fn get_feature_data() -> (Vec<FeatureTag>, HashMap<String, Vec<CardFeature>>) {
    // feature_conditions関数からデータを取得
    let feature_map = datapack::feature_conditions();

    // JsValueからHashMapに変換
    let feature_map: HashMap<String, Vec<serde_json::Value>> =
        serde_wasm_bindgen::from_value(feature_map).unwrap_or_default();

    let mut feature_tags = Vec::new();
    let mut features_by_tag = HashMap::new();

    for (tag_key, features_json) in feature_map {
        // FeatureTagを作成（先頭2文字削除）
        let display_name = if tag_key.len() >= 2 {
            tag_key[2..].to_string()
        } else {
            tag_key.clone()
        };

        feature_tags.push(FeatureTag {
            id: tag_key.clone(),
            display_name,
        });

        // CardFeatureのリストを作成
        let card_features: Vec<CardFeature> = features_json
            .into_iter()
            .filter_map(|feature_json| {
                let name = feature_json.get("name")?.as_str()?.to_string();
                Some(CardFeature { name })
            })
            .collect();

        features_by_tag.insert(tag_key, card_features);
    }

    // ソート（表示順序を統一 - 数値順）
    feature_tags.sort_by(|a, b| {
        // idの先頭2文字（数値部分）で比較
        let a_num = a.id.chars().take(2).collect::<String>();
        let b_num = b.id.chars().take(2).collect::<String>();
        a_num.cmp(&b_num)
    });

    (feature_tags, features_by_tag)
}

#[component]
pub fn FeatureOverlay(
    selected_features: RwSignal<HashMap<String, bool>>,
    on_feature_change: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let (feature_tags, features_by_tag) = get_feature_data();

    // 現在開いているカテゴリのID
    let (open_category, set_open_category) = signal::<Option<String>>(None);

    let toggle_category = move |tag_id: String| {
        set_open_category.update(|current| {
            if current.as_ref() == Some(&tag_id) {
                *current = None; // 既に開いている場合は閉じる
            } else {
                *current = Some(tag_id); // 他のを閉じて開く
            }
        });
    };

    let toggle_card_feature = move |feature_name: String| {
        selected_features.update(|features| {
            let current = features.get(&feature_name).copied().unwrap_or(false);
            if current {
                features.remove(&feature_name);
            } else {
                features.insert(feature_name, true);
            }
        });

        // 親に現在選択されているfeature名のリストを通知
        let selected_names: Vec<String> = selected_features
            .read()
            .iter()
            .filter_map(|(name, &selected)| if selected { Some(name.clone()) } else { None })
            .collect();
        on_feature_change.set(selected_names);
    };

    view! {
        <div class="space-y-4">
            {feature_tags.into_iter().map(|tag| {
                let tag_id = tag.id.clone();
                let tag_id_for_toggle = tag.id.clone();
                let tag_id_for_open_check = tag.id.clone();
                let tag_id_for_features = tag.id.clone();
                let features_by_tag_clone = features_by_tag.clone();
                let features_by_tag_clone2 = features_by_tag.clone();

                let is_open = Memo::new(move |_| {
                    open_category.get().as_ref() == Some(&tag_id_for_open_check)
                });

                let has_selected_features = Memo::new(move |_| {
                    let selected = selected_features.read();
                    if let Some(card_features) = features_by_tag_clone.get(&tag_id_for_features) {
                        card_features.iter()
                            .any(|f| selected.get(&f.name).copied().unwrap_or(false))
                    } else {
                        false
                    }
                });

                view! {
                    <div class="border rounded-lg">
                        <button
                            class={move || {
                                let base = "w-full px-4 py-3 text-left font-medium rounded-lg transition-colors";
                                let state_class = if has_selected_features.get() {
                                    "bg-blue-50 text-blue-700 border-blue-200"
                                } else if is_open.get() {
                                    "bg-gray-50 text-gray-700"
                                } else {
                                    "bg-white text-gray-700 hover:bg-gray-50"
                                };
                                format!("{base} {state_class}")
                            }}
                            on:click=move |_| toggle_category(tag_id_for_toggle.clone())
                        >
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <Show when=move || has_selected_features.get()>
                                        <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
                                    </Show>
                                    <span>{tag.display_name}</span>
                                </div>
                                <span class={move || if is_open.get() { "transform rotate-180" } else { "" }}>
                                    "▼"
                                </span>
                            </div>
                        </button>

                        <Show when=move || is_open.get()>
                            <div class="border-t bg-gray-50">
                                <div class="p-3 space-y-2">
                                    {if let Some(card_features) = features_by_tag_clone2.get(&tag_id) {
                                        if card_features.is_empty() {
                                            view! { <div class="text-gray-500 text-sm p-2">"No features"</div> }.into_any()
                                        } else {
                                            card_features.iter().map(|feature| {
                                                let feature_name = feature.name.clone();
                                                let feature_name_for_toggle = feature.name.clone();
                                                let feature_name_for_check = feature.name.clone();

                                                let is_selected = Signal::derive(move || {
                                                    selected_features.read().get(&feature_name_for_check).copied().unwrap_or(false)
                                                });

                                                view! {
                                                    <SvgToggleSwitch
                                                        is_on=move || is_selected.get()
                                                        on_toggle=move |_new_state| toggle_card_feature(feature_name_for_toggle.clone())
                                                        label=feature_name
                                                    />
                                                }
                                            }).collect_view().into_any()
                                        }
                                    } else {
                                        view! { <div class="text-gray-500 text-sm p-2">"No features"</div> }.into_any()
                                    }}
                                </div>
                            </div>
                        </Show>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
