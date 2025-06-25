use crate::components::SvgToggleSwitch;
use crate::types::CardTypeFilter;
use datapack::gen::card_types::{CARD_TYPES, EXTENDED_CARD_TYPES, PRIMARY_CARD_TYPES};
use leptos::prelude::*;

// カードタイプトグルスイッチを生成する関数
fn create_card_type_toggle(
    code: &'static str,
    name: &'static str,
    card_type_filter: ReadSignal<CardTypeFilter>,
    set_card_type_filter: WriteSignal<CardTypeFilter>,
) -> impl IntoView {
    view! {
        <SvgToggleSwitch
            is_on=move || {
                let filter = card_type_filter.get();
                filter.is_selected_by_code(code)
            }
            on_toggle=move |new_state| {
                set_card_type_filter.update(|filter| {
                    filter.set_by_code(code, new_state);
                });
            }
            label=name.to_string()
        />
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
            <div class="flex items-center mb-3">
                <h3 class="text-lg font-semibold text-gray-800">"カード種別"</h3>

                // 展開/折りたたみボタン
                <div class="flex items-center ml-4">
                    <button
                        type="button"
                        class="flex items-center space-x-1 text-sm transition-colors"
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

                    // 拡張項目が選択されている場合の説明メッセージ
                    {move || {
                        if card_type_filter.get().has_extended_selection() {
                            view! {
                                <div class="ml-2 text-xs text-gray-500 flex items-center">
                                    "(拡張項目選択中)"
                                </div>
                            }.into_any()
                        } else {
                            view! { <div /> }.into_any()
                        }
                    }}
                </div>
            </div>

            // デフォルト表示のカード種別（動的生成）
            <div class="grid grid-cols-3 gap-3">
                {
                    PRIMARY_CARD_TYPES.iter().map(|&code| {
                        let card_type_info = CARD_TYPES.iter().find(|ct| ct.code == code);
                        if let Some(info) = card_type_info {
                            create_card_type_toggle(info.code, info.name, card_type_filter, set_card_type_filter).into_any()
                        } else {
                            view! { <div /> }.into_any()
                        }
                    }).collect::<Vec<_>>()
                }
            </div>

            // 拡張カード種別（条件付き表示・動的生成）
            {move || {
                if should_show_extended.get() {
                    view! {
                        <div class="mt-3 grid grid-cols-3 gap-3">
                            <hr class="col-span-3 border-gray-300 my-2" />

                            {
                                EXTENDED_CARD_TYPES.iter().map(|&code| {
                                    let card_type_info = CARD_TYPES.iter().find(|ct| ct.code == code);
                                    if let Some(info) = card_type_info {
                                        create_card_type_toggle(info.code, info.name, card_type_filter, set_card_type_filter).into_any()
                                    } else {
                                        view! { <div /> }.into_any()
                                    }
                                }).collect::<Vec<_>>()
                            }
                        </div>
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}
