use crate::components::SvgToggleSwitch;
use crate::types::LBFilter;
use leptos::prelude::*;
use leptos::*;
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
    lb_filter: ReadSignal<LBFilter>,
    set_lb_filter: WriteSignal<LBFilter>,
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
        // トグル前の選択数を記録
        let had_selections_before = selected_burst_features.read().values().any(|&v| v);

        let feature_was_turned_on = {
            let mut was_turned_on = false;
            selected_burst_features.update(|features| {
                let current = features.get(&feature_name).copied().unwrap_or(false);
                if current {
                    features.remove(&feature_name);
                } else {
                    features.insert(feature_name, true);
                    was_turned_on = true; // フィーチャーがONになった
                }
            });
            was_turned_on
        };

        // トグル後の選択数を確認
        let has_selections_after = selected_burst_features.read().values().any(|&v| v);

        // 1つ以上から0個になった場合のイベント
        if had_selections_before && !has_selections_after {
            leptos::logging::log!("All burst features were deselected - switching to 指定なし");
            // LBフィルタを「指定なし」に自動切り替え
            set_lb_filter.update(|filter| {
                filter.set_selection(0); // 「指定なし」に設定
            });
        }

        // LBフィーチャーがONになった場合、LBフィルタを「LBあり」に自動切り替え
        if feature_was_turned_on {
            let current_lb_selection = lb_filter.get().selection;
            if current_lb_selection != 1 {
                // 「LBあり」でない場合
                set_lb_filter.update(|filter| {
                    filter.set_selection(1); // 「LBあり」に設定
                });
                leptos::logging::log!("Auto-switched to LBあり due to burst feature selection");
            }
        }

        // 親に現在選択されているburst feature名のリストを通知
        let selected_names: Vec<String> = selected_burst_features
            .read()
            .iter()
            .filter_map(|(name, &selected)| if selected { Some(name.clone()) } else { None })
            .collect();
        on_burst_feature_change.set(selected_names);
    };

    // LBフィルタの操作機能
    let set_lb_selection = move |value: u8| {
        let should_clear_burst_features = {
            let current_selection = lb_filter.get().selection;
            // トグル機能: 同じボタンをクリックしたらクリア、そうでなければ新しい値を設定
            if current_selection == value {
                // 同じボタンなのでクリア
                set_lb_filter.update(|filter| filter.clear());
                false
            } else {
                // 新しい値を設定
                set_lb_filter.update(|filter| filter.set_selection(value));
                value == 2 // LBなしの場合はバースト効果をクリア
            }
        };

        // 「LBなし」を選択した場合、バースト効果選択をクリア
        if should_clear_burst_features {
            // デバッグ: クリア前の状態をログ出力
            leptos::logging::log!(
                "Clearing burst features. Before: {:?}",
                selected_burst_features.read().len()
            );

            selected_burst_features.set(HashMap::new());
            on_burst_feature_change.set(Vec::new());

            // デバッグ: クリア後の状態をログ出力
            leptos::logging::log!(
                "Cleared burst features. After: {:?}",
                selected_burst_features.read().len()
            );
        }
    };

    // LBフィルタボタンコンポーネント
    let lb_button = move |value: u8,
                          label: &'static str,
                          inactive_class: &'static str,
                          active_class: &'static str| {
        let is_active = Memo::new(move |_| lb_filter.get().selection == value);

        view! {
            <button
                class=move || {
                    let base = "px-4 py-2 text-sm font-semibold rounded-md border-2 transition-all duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-offset-2";
                    if is_active.get() {
                        format!("{} {} shadow-md transform scale-105", base, active_class)
                    } else {
                        format!("{} {} hover:shadow-sm hover:scale-102", base, inactive_class)
                    }
                }
                on:click=move |_| set_lb_selection(value)
            >
                {label}
            </button>
        }
    };

    // クリアボタンの表示判定
    let should_show_clear = Memo::new(move |_| {
        !selected_burst_features.read().is_empty() || lb_filter.get().has_any()
    });

    // クリアボタンの処理
    let clear_all = move |_| {
        selected_burst_features.set(HashMap::new());
        on_burst_feature_change.set(Vec::new());
        set_lb_filter.update(|filter| {
            filter.clear();
        });
    };

    view! {
        <div class="p-6 max-h-[60vh] overflow-y-auto">
            <div class="flex items-center justify-between mb-4">
                // LBフィルタ部分
                <div class="flex items-center gap-3 mb-4">
                    {lb_button(
                        0,
                        "指定なし",
                        "bg-gray-50 border-gray-300 text-gray-600 hover:bg-gray-100 hover:border-gray-400 focus:ring-gray-300",
                        "bg-gray-600 border-gray-600 text-white focus:ring-gray-400"
                    )}
                    {lb_button(
                        1,
                        "LBあり",
                        "bg-blue-50 border-blue-300 text-blue-700 hover:bg-blue-100 hover:border-blue-400 focus:ring-blue-300",
                        "bg-blue-600 border-blue-600 text-white focus:ring-blue-400"
                    )}
                    {lb_button(
                        2,
                        "LBなし",
                        "bg-red-50 border-red-300 text-red-700 hover:bg-red-100 hover:border-red-400 focus:ring-red-300",
                        "bg-red-600 border-red-600 text-white focus:ring-red-400"
                    )}
                </div>

                <Show when=move || should_show_clear.get()>
                    <button
                        class="px-3 py-1 text-sm font-medium text-gray-600 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 hover:text-gray-700 transition-colors"
                        on:click=clear_all
                    >
                        "クリア"
                    </button>
                </Show>
            </div>


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
                        let selected_count = Memo::new(move |_| {
                            selected_burst_features.read()
                                .iter()
                                .filter(|(feature_name, &is_selected)| {
                                    is_selected && features_for_count.iter().any(|f| &f.name == *feature_name)
                                })
                                .count()
                        });

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
                                            let count = selected_count.get();
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

                                                                let is_selected = Memo::new(move |_| {
                                                                    selected_burst_features.read()
                                                                        .get(&feature_name_for_selected)
                                                                        .copied()
                                                                        .unwrap_or(false)
                                                                });

                                                                view! {
                                                                    <SvgToggleSwitch
                                                                        is_on=move || is_selected.get()
                                                                        on_toggle=move |_new_state| toggle_burst_feature(feature_name_for_click.clone())
                                                                        label=feature_name
                                                                    />
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
