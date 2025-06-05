use leptos::prelude::*;
use datapack::CardExport;

#[component]
pub fn CardList(cards: Vec<CardExport>) -> impl IntoView {
    let is_empty = cards.is_empty();
    
    view! {
        <div class="bg-white rounded-lg shadow">
            <div class="divide-y divide-gray-200">
                {cards.into_iter().map(|card| {
                    view! {
                        <div class="card-item">
                            <div class="flex justify-between items-start">
                                <div class="flex-1">
                                    <h3 class="font-semibold text-lg">
                                        {card.name()}
                                    </h3>
                                    <p class="text-sm text-gray-600 mt-1">
                                        {card.code()}
                                    </p>
                                    {
                                        let skill_text = card.skill_text();
                                        if !skill_text.is_empty() {
                                            view! {
                                                <div class="mt-2 text-sm whitespace-pre-wrap">
                                                    {skill_text}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                    {
                                        let burst_text = card.burst_text();
                                        let has_burst = card.has_burst() > 0;
                                        if has_burst && !burst_text.is_empty() {
                                            view! {
                                                <div class="mt-2 text-sm text-orange-600">
                                                    <span class="font-semibold">"【ライフバースト】"</span>
                                                    {burst_text}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                </div>
                                <div class="ml-4 text-right">
                                    {
                                        let level = card.level();
                                        if !level.is_empty() {
                                            view! {
                                                <div class="text-sm text-gray-600">
                                                    {format!("Lv.{}", level)}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                    {
                                        let power = card.power();
                                        if !power.is_empty() {
                                            view! {
                                                <div class="text-sm">
                                                    {format!("Power: {}", power)}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                    {
                                        let cost = card.cost();
                                        if !cost.is_empty() {
                                            view! {
                                                <div class="text-sm">
                                                    {format!("Cost: {}", cost)}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                </div>
                            </div>
                        </div>
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