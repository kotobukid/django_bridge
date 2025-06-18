use crate::types::CardTypeFilter;
use leptos::prelude::*;

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
            
            // デフォルト表示のカード種別
            <div class="grid grid-cols-3 gap-2">
                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().lrig
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.lrig = checked);
                        }
                    />
                    <span>"ルリグ"</span>
                </label>

                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().lrig_assist
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.lrig_assist = checked);
                        }
                    />
                    <span>"アシスト"</span>
                </label>

                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().arts
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.arts = checked);
                        }
                    />
                    <span>"アーツ"</span>
                </label>

                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().signi
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.signi = checked);
                        }
                    />
                    <span>"シグニ"</span>
                </label>

                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().spell
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.spell = checked);
                        }
                    />
                    <span>"スペル"</span>
                </label>

                <label class="flex items-center space-x-2 text-sm">
                    <input
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                        prop:checked=move || card_type_filter.get().piece
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_card_type_filter.update(|filter| filter.piece = checked);
                        }
                    />
                    <span>"ピース"</span>
                </label>
            </div>

            // 拡張カード種別（条件付き表示）
            {move || {
                if should_show_extended.get() {
                    view! {
                        <div class="mt-3 grid grid-cols-3 gap-2">
                            <hr class="col-span-3 border-gray-300 my-2" />
                            
                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().key
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.key = checked);
                                    }
                                />
                                <span>"キー"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().resona
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.resona = checked);
                                    }
                                />
                                <span>"レゾナ"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().signi_craft
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.signi_craft = checked);
                                    }
                                />
                                <span>"クラフトシグニ"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().spell_craft
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.spell_craft = checked);
                                    }
                                />
                                <span>"クラフトスペル"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().arts_craft
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.arts_craft = checked);
                                    }
                                />
                                <span>"クラフトアーツ"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().resona_craft
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.resona_craft = checked);
                                    }
                                />
                                <span>"クラフトレゾナ"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().piece_relay
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.piece_relay = checked);
                                    }
                                />
                                <span>"リレーピース"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().piece_craft
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.piece_craft = checked);
                                    }
                                />
                                <span>"クラフトピース"</span>
                            </label>

                            <label class="flex items-center space-x-2 text-sm">
                                <input
                                    type="checkbox"
                                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                                    prop:checked=move || card_type_filter.get().token
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        set_card_type_filter.update(|filter| filter.token = checked);
                                    }
                                />
                                <span>"トークン"</span>
                            </label>
                        </div>
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}