use crate::types::CardTypeFilter;
use datapack::gen::card_types::{CARD_TYPES, EXTENDED_CARD_TYPES, PRIMARY_CARD_TYPES};
use leptos::prelude::*;

// カードタイプボタンを生成する関数
fn create_card_type_button(
    code: &'static str,
    name: &'static str,
    card_type_filter: ReadSignal<CardTypeFilter>,
    set_card_type_filter: WriteSignal<CardTypeFilter>,
) -> impl IntoView {
    view! {
        <button
            class=move || {
                if card_type_filter.get().is_selected_by_code(code) {
                    "px-3 py-1.5 text-sm rounded-md bg-blue-600 text-white border border-blue-600 hover:bg-blue-700 transition-colors"
                } else {
                    "px-3 py-1.5 text-sm rounded-md bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 transition-colors"
                }
            }
            on:click=move |_| {
                set_card_type_filter.update(|filter| {
                    let current_state = filter.is_selected_by_code(code);
                    filter.set_by_code(code, !current_state);
                });
            }
        >
            {name}
        </button>
    }
}

#[component]
pub fn CardTypeSelector(
    card_type_filter: ReadSignal<CardTypeFilter>,
    set_card_type_filter: WriteSignal<CardTypeFilter>,
) -> impl IntoView {
    // 展開状態の管理
    let (is_expanded, set_is_expanded) = signal(false);

    // 拡張項目が選択されている場合は展開状態を維持
    let should_show_extended = Signal::derive(move || {
        is_expanded.get() || card_type_filter.get().has_extended_selection()
    });

    view! {
        <div class="bg-white p-4 rounded-lg shadow-sm border border-gray-200">
            // 拡張項目選択中の説明
            {move || {
                if card_type_filter.get().has_extended_selection() {
                    view! {
                        <div class="mb-2 text-xs text-gray-500">
                            "(拡張項目選択中)"
                        </div>
                    }.into_any()
                } else {
                    view! {};
                    ().into_any()
                }
            }}

            // デフォルト表示のカード種別（ボタンスタイル）+ 展開ボタン
            <div class="flex flex-wrap gap-2 justify-between items-center">
                <div class="flex flex-wrap gap-2">
                    {
                        PRIMARY_CARD_TYPES.iter().map(|&code| {
                            let card_type_info = CARD_TYPES.iter().find(|ct| ct.code == code);
                            if let Some(info) = card_type_info {
                                create_card_type_button(info.code, info.name, card_type_filter, set_card_type_filter).into_any()
                            } else {
                                view! { <div /> }.into_any()
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>

                // 展開/折りたたみボタン（右端）
                <button
                    type="button"
                    class="flex items-center space-x-1 text-sm transition-colors ml-2"
                    class:text-blue-600=move || !card_type_filter.get().has_extended_selection()
                    class:hover:text-blue-800=move || !card_type_filter.get().has_extended_selection()
                    class:text-gray-400=move || card_type_filter.get().has_extended_selection()
                    class:cursor-not-allowed=move || card_type_filter.get().has_extended_selection()
                    on:click=move |_| {
                        // 拡張項目が選択されている場合は折りたたみを無効化
                        if !card_type_filter.get().has_extended_selection() {
                            set_is_expanded.update(|v| *v = !*v);
                        }
                    }
                >
                    <span class="text-lg" inner_html=move || {
                        if should_show_extended.get() {
                            "▼"
                        } else {
                            "▶"
                        }
                    }></span>
                    <span>{move || {
                        if should_show_extended.get() {
                            "閉じる"
                        } else {
                            "もっと見る"
                        }
                    }}</span>
                </button>
            </div>

            // 拡張カード種別（条件付き表示・ボタンスタイル）
            {move || {
                if should_show_extended.get() {
                    view! {
                        <div class="mt-3">
                            <hr class="border-gray-300 mb-3" />
                            <div class="flex flex-wrap gap-2">
                                {
                                    EXTENDED_CARD_TYPES.iter().map(|&code| {
                                        let card_type_info = CARD_TYPES.iter().find(|ct| ct.code == code);
                                        if let Some(info) = card_type_info {
                                            create_card_type_button(info.code, info.name, card_type_filter, set_card_type_filter).into_any()
                                        } else {
                                            view! { <div /> }.into_any()
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}
