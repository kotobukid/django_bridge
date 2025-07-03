use crate::components::card_item::{CardItem, ViewMode};
use crate::utils::maintenance::is_maintenance_mode;
use datapack::CardExport;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

// Helper function to search cards by different criteria
fn search_cards_by_param(param: &str) -> Vec<CardExport> {
    let cards = match datapack::get_all_cards() {
        Ok(cards) => cards,
        Err(_) => return Vec::new(),
    };

    // Try different search methods
    let mut results = Vec::new();

    // 1. Search by ID (exact match)
    if let Ok(id) = param.parse::<i32>() {
        results.extend(cards.iter().filter(|card| card.id() == id).cloned());
    }

    // 2. Search by code (exact match)
    results.extend(
        cards
            .iter()
            .filter(|card| card.code().eq_ignore_ascii_case(param))
            .cloned(),
    );

    // 3. Search by pronunciation (exact match)
    results.extend(
        cards
            .iter()
            .filter(|card| {
                let full_name = card.name();
                // Extract pronunciation part (inside <> if present)
                if let Some(start) = full_name.find(" <") {
                    if let Some(end) = full_name.find(">") {
                        let pronunciation = &full_name[start + 2..end];
                        return pronunciation.eq_ignore_ascii_case(param);
                    }
                }
                false
            })
            .cloned(),
    );

    // 4. Search by card name (partial match)
    results.extend(
        cards
            .iter()
            .filter(|card| {
                let full_name = card.name();
                // Extract card name part (before <> if present)
                let card_name = if let Some(bracket_pos) = full_name.find(" <") {
                    &full_name[..bracket_pos]
                } else {
                    &full_name
                };
                card_name.contains(param)
            })
            .cloned(),
    );

    // Remove duplicates by converting to a set-like structure
    let mut unique_results = Vec::new();
    for card in results {
        if !unique_results
            .iter()
            .any(|c: &CardExport| c.id() == card.id())
        {
            unique_results.push(card);
        }
    }

    unique_results
}

#[component]
pub fn CardDetailPage() -> impl IntoView {
    let params = use_params_map();

    let cards = Signal::derive(move || {
        let param_map = params.get();
        if let Some(param_value) = param_map.get("param") {
            search_cards_by_param(&param_value)
        } else {
            Vec::new()
        }
    });

    let param_value = Signal::derive(move || params.get().get("param").unwrap_or_default());

    view! {
        <div class="container mx-auto px-4 py-8">
            <div class="mb-6">
                <h1 class="text-3xl font-bold text-gray-900 mb-2">
                    "Card Search Results"
                </h1>
                <p class="text-gray-600">
                    "Search parameter: " <span class="font-mono bg-gray-100 px-2 py-1 rounded">{param_value}</span>
                </p>
            </div>

            {move || {
                let card_list = cards.get();
                let card_count = card_list.len();

                if card_count == 0 {
                    view! {
                        <div class="bg-white rounded-lg shadow p-8 text-center">
                            <div class="text-gray-500 text-lg">
                                "No cards found matching: " <span class="font-mono">{param_value.get()}</span>
                            </div>
                            <div class="mt-4 text-sm text-gray-400">
                                "Search criteria: ID, Code, Pronunciation, or Card Name"
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // Get pronunciation for edit link before consuming card_list
                    let edit_pronunciation = if is_maintenance_mode() && !card_list.is_empty() {
                        let first_card = &card_list[0];
                        let full_name = first_card.name();
                        if let Some(start) = full_name.find(" <") {
                            full_name.find(">").map(|end| full_name[start + 2..end].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    view! {
                        <div>
                            <div class="mb-4 flex justify-between items-center">
                                <div class="text-sm text-gray-600">
                                    {format!("Found {} card{}", card_count, if card_count == 1 { "" } else { "s" })}
                                </div>
                                {move || {
                                    if let Some(pronunciation) = &edit_pronunciation {
                                        if !pronunciation.is_empty() {
                                            view! {
                                                <a
                                                    href=format!("/edit/{}", pronunciation)
                                                    class="inline-flex items-center px-3 py-1 bg-yellow-500 text-white text-sm rounded hover:bg-yellow-600 transition-colors"
                                                >
                                                    "🛠️ Edit Features"
                                                </a>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </div>
                            <div class="space-y-4">
                                {card_list.into_iter().map(|card| {
                                    view! {
                                        <CardItem card=card view_mode=Signal::derive(|| ViewMode::Detailed) />
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }.into_any()
                }
            }}

            <div class="mt-8 text-center">
                <a
                    href="/"
                    class="inline-flex items-center px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
                >
                    "← Back to Home"
                </a>
            </div>
        </div>
    }
}
