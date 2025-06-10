use crate::components::card_item::CardItem;
use datapack::CardExport;
use leptos::prelude::*;

#[component]
pub fn CardList(cards: Vec<CardExport>) -> impl IntoView {
    let is_empty = cards.is_empty();
    
    view! {
        <div class="bg-white rounded-lg shadow">
            <div class="p-4">
                {cards.into_iter().map(|card| {
                    view! {
                        <CardItem card=card />
                    }
                }).collect_view()}
            </div>

            <Show when=move || is_empty>
                <div class="p-8 text-center text-gray-500">
                    "No cards found matching your filters."
                </div>
            </Show>
        </div>
    }
}
