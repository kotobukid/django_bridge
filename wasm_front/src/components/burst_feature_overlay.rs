use leptos::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BurstFeatureTag {
    id: String,
    display_name: String,
}

#[derive(Debug, Clone)]
struct BurstFeature {
    name: String,
}

// 実際のdatapackからBurstFeatureデータを取得する関数
fn get_burst_feature_data() -> (Vec<BurstFeatureTag>, HashMap<String, Vec<BurstFeature>>) {
    // burst_feature_conditions関数からデータを取得
    let feature_map = datapack::burst_feature_conditions();

    // JsValueからHashMapに変換
    let feature_map: HashMap<String, Vec<serde_json::Value>> =
        serde_wasm_bindgen::from_value(feature_map).unwrap_or_default();

    let mut feature_tags = Vec::new();
    let mut features_by_tag = HashMap::new();

    for (tag_key, features_json) in feature_map {
        // BurstFeatureTagを作成（先頭2文字削除）
        let display_name = if tag_key.len() >= 2 {
            tag_key[2..].to_string()
        } else {
            tag_key.clone()
        };

        feature_tags.push(BurstFeatureTag {
            id: tag_key.clone(),
            display_name,
        });

        // BurstFeatureのリストを作成
        let burst_features: Vec<BurstFeature> = features_json
            .into_iter()
            .filter_map(|feature_json| {
                let name = feature_json.get("name")?.as_str()?.to_string();
                Some(BurstFeature { name })
            })
            .collect();

        features_by_tag.insert(tag_key, burst_features);
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
pub fn BurstFeatureOverlay(
    selected_burst_features: RwSignal<HashMap<String, bool>>,
    on_burst_feature_change: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let (feature_tags, features_by_tag) = get_burst_feature_data();

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

    let toggle_burst_feature = move |feature_name: String| {
        selected_burst_features.update(|features| {
            let current = features.get(&feature_name).copied().unwrap_or(false);
            if current {
                features.remove(&feature_name);
            } else {
                features.insert(feature_name, true);
            }
        });

        // 親に現在選択されているburst feature名のリストを通知
        let selected_names: Vec<String> = selected_burst_features
            .read()
            .iter()
            .filter_map(|(name, &selected)| if selected { Some(name.clone()) } else { None })
            .collect();
        on_burst_feature_change.set(selected_names);
    };

    view! {
        <div class="p-6 max-h-[60vh] overflow-y-auto">
            <h3 class="text-lg font-bold text-gray-900 mb-4">"ライフバースト効果で絞り込み"</h3>
            
            <div class="space-y-2">
                {feature_tags
                    .into_iter()
                    .map(|tag| {
                        let tag_id = tag.id.clone();
                        let tag_display = tag.display_name.clone();
                        let features = features_by_tag.get(&tag.id).cloned().unwrap_or_default();
                        
                        // クローンを作成してクロージャで使用
                        let features_for_count = features.clone();
                        let tag_id_for_open = tag_id.clone();
                        let tag_id_for_click = tag_id.clone();
                        
                        // このタグ内で選択されているBurstFeatureの数をカウント
                        let selected_count = move || {
                            selected_burst_features.read()
                                .iter()
                                .filter(|(feature_name, &is_selected)| {
                                    is_selected && features_for_count.iter().any(|f| &f.name == *feature_name)
                                })
                                .count()
                        };

                        // is_openをRcで共有可能にする代わりに、直接実装
                        let is_open_fn = {
                            let tag_id_check = tag_id.clone();
                            move || open_category.get().as_ref() == Some(&tag_id_check)
                        };

                        view! {
                            <div class="border border-gray-200 rounded-md bg-white">
                                // カテゴリヘッダー
                                <button
                                    class="w-full p-3 text-left flex items-center justify-between hover:bg-gray-50 transition-colors"
                                    on:click=move |_| toggle_category(tag_id_for_click.clone())
                                >
                                    <div class="flex items-center space-x-2">
                                        <span class="font-medium text-gray-700">{tag_display.clone()}</span>
                                        {move || {
                                            let count = selected_count();
                                            if count > 0 {
                                                view! {
                                                    <span class="bg-blue-100 text-blue-800 text-xs font-medium px-2 py-1 rounded-full">
                                                        {count}
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                    </div>
                                    <svg 
                                        class=move || format!("w-5 h-5 text-gray-400 transition-transform {}", 
                                            if is_open_fn() { "rotate-180" } else { "" }
                                        )
                                        fill="none" 
                                        stroke="currentColor" 
                                        viewBox="0 0 24 24"
                                    >
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                    </svg>
                                </button>

                                // BurstFeatureリスト（アコーディオン）
                                {
                                    let features_clone = features.clone();
                                    let tag_id_for_condition = tag_id.clone();
                                    move || {
                                        if open_category.get().as_ref() == Some(&tag_id_for_condition) {
                                            view! {
                                                <div class="border-t border-gray-200 p-3 bg-gray-50">
                                                    <div class="grid grid-cols-1 gap-2">
                                                        {features_clone
                                                            .iter()
                                                            .map(|feature| {
                                                                let feature_name = feature.name.clone();
                                                                let feature_name_for_click = feature_name.clone();
                                                                let feature_name_for_selected = feature_name.clone();
                                                                
                                                                let is_selected = move || {
                                                                    selected_burst_features.read()
                                                                        .get(&feature_name_for_selected)
                                                                        .copied()
                                                                        .unwrap_or(false)
                                                                };

                                                                view! {
                                                                    <label class="flex items-center space-x-2 cursor-pointer hover:bg-white rounded p-2 transition-colors">
                                                                        <input
                                                                            type="checkbox"
                                                                            class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
                                                                            checked=is_selected
                                                                            on:change=move |_| toggle_burst_feature(feature_name_for_click.clone())
                                                                        />
                                                                        <span class="text-sm text-gray-700">{feature_name}</span>
                                                                    </label>
                                                                }
                                                            })
                                                            .collect::<Vec<_>>()
                                                        }
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                }
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
        </div>
    }
}