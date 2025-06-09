use crate::types::CardTypeFilter;
use leptos::prelude::*;

#[component]
pub fn CardTypeSelector(
    card_type_filter: ReadSignal<CardTypeFilter>,
    set_card_type_filter: WriteSignal<CardTypeFilter>,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 rounded-lg shadow-sm border border-gray-200">
            <h3 class="text-lg font-semibold mb-3 text-gray-800">"カード種別"</h3>
            <div class="grid grid-cols-3 gap-2">
                // 主要カード種別
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

                // クラフト系カード種別
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
        </div>
    }
}