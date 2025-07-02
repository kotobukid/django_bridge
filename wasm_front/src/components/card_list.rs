use crate::components::card_item::{CardItem, ViewMode};
use datapack::CardExport;
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn CardList(
    cards: Vec<CardExport>, 
    total_count: usize,
    override_pronunciations: ReadSignal<HashSet<String>>,
) -> impl IntoView {
    let is_empty = cards.is_empty();
    // Load initial view mode from localStorage, default to Compact
    let initial_mode = ViewMode::load_from_storage();
    let (view_mode, set_view_mode) = signal(initial_mode);

    // Create an effect to save to localStorage whenever view_mode changes
    Effect::new(move |_| {
        let current_mode = view_mode.get();
        current_mode.save_to_storage();
    });

    view! {
        <div class="bg-white rounded-lg shadow">
            <Show when=move || !is_empty>
                <div class="px-4 py-3 border-b border-gray-200">
                    <div class="flex items-center justify-between">
                        <span class="text-sm text-gray-600">
                            {format!("Found {} cards", total_count)}
                        </span>
                        <div class="flex items-center gap-2">
                            <span class="text-sm text-gray-600">View:</span>
                            <div class="relative inline-flex bg-gray-100 rounded-lg p-1">
                                <button
                                    class=move || format!(
                                        "px-3 py-1 text-sm font-medium rounded-md transition-colors {}",
                                        if view_mode.get() == ViewMode::Compact {
                                            "bg-white text-gray-900 shadow-sm"
                                        } else {
                                            "text-gray-500 hover:text-gray-700"
                                        }
                                    )
                                    on:click=move |_| set_view_mode.set(ViewMode::Compact)
                                >
                                    "Compact"
                                </button>
                                <button
                                    class=move || format!(
                                        "px-3 py-1 text-sm font-medium rounded-md transition-colors {}",
                                        if view_mode.get() == ViewMode::Detailed {
                                            "bg-white text-gray-900 shadow-sm"
                                        } else {
                                            "text-gray-500 hover:text-gray-700"
                                        }
                                    )
                                    on:click=move |_| set_view_mode.set(ViewMode::Detailed)
                                >
                                    "Detailed"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>

            <div class="p-4">
                {move || {
                    let override_set = override_pronunciations.get();
                    leptos::logging::log!("CardList render: override_set has {} items", override_set.len());
                    
                    cards.clone().into_iter().map(|card| {
                        let pronunciation = card.pronunciation();
                        let has_override = override_set.contains(&pronunciation);
                        
                        // デバッグ：すべてのカードの判定を出力
                        if override_set.len() > 0 && (pronunciation.contains("エンケンノマイ") || pronunciation.contains("バブルスボマー") || pronunciation.contains("ミーンナデハジメルアイドルライフ")) {
                            leptos::logging::log!("Card pronunciation: '{}', has_override: {}", pronunciation, has_override);
                        }
                        
                        view! {
                            <CardItem 
                                card=card 
                                view_mode=view_mode.into()
                                has_override=has_override
                            />
                        }
                    }).collect_view()
                }}
            </div>

            <Show when=move || is_empty>
                <div class="p-8 text-center text-gray-500">
                    "No cards found matching your filters."
                </div>
            </Show>
        </div>
    }
}
