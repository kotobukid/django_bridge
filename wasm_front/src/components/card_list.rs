use crate::components::svg_definition::{
    IconBlack, IconBlue, IconColorless, IconGreen, IconRed, IconWhite,
};
use datapack::CardExport;
use leptos::prelude::*;

fn get_colors_from_bits(bits: i32) -> Vec<i32> {
    let mut colors = Vec::new();
    if bits & (1 << 1) != 0 {
        colors.push(1);
    } // White
    if bits & (1 << 2) != 0 {
        colors.push(2);
    } // Blue
    if bits & (1 << 3) != 0 {
        colors.push(3);
    } // Red
    if bits & (1 << 4) != 0 {
        colors.push(4);
    } // Black
    if bits & (1 << 5) != 0 {
        colors.push(5);
    } // Green
    if bits & (1 << 6) != 0 {
        colors.push(6);
    } // Colorless
    colors
}

fn render_color_icon(color: i32) -> impl IntoView {
    match color {
        1 => view! { <IconWhite /> }.into_any(),
        2 => view! { <IconBlue /> }.into_any(),
        3 => view! { <IconRed /> }.into_any(),
        4 => view! { <IconBlack /> }.into_any(),
        5 => view! { <IconGreen /> }.into_any(),
        6 => view! { <IconColorless /> }.into_any(),
        _ => view! { <span></span> }.into_any(),
    }
}

#[component]
pub fn CardList(cards: Vec<CardExport>) -> impl IntoView {
    let is_empty = cards.is_empty();

    view! {
        <div class="bg-white rounded-lg shadow">
            <div class="p-4">
                {cards.into_iter().map(|card| {
                    let color_style = datapack::bits_to_gradient_native(card.color() as i32);

                    let card_url = format!("https://www.takaratomy.co.jp/products/wixoss/card_list.php?card=card_detail&card_no={}", card.code());
                    view! {
                        <div class="card-item" style=format!("{}; border-radius: 8px; padding: 16px; margin: 8px 0; border: 1px solid rgba(0,0,0,0.1);", color_style)>
                            <div class="flex justify-between items-start">
                                <div class="flex-1">
                                    <h3 class="font-semibold text-lg" style="color: #374151;">
                                        <a href=card_url target="_blank" class="flex items-center gap-1">
                                            {
                                                let colors = get_colors_from_bits(card.color() as i32);
                                                colors.into_iter().map(|color| render_color_icon(color)).collect_view()
                                            }
                                            {card.name()}
                                        </a>
                                    </h3>
                                    <p class="text-sm mt-1" style="color: #374151; opacity: 0.8;">
                                        {card.code()}
                                    </p>
                                    {
                                        let skill_text = card.skill_text();
                                        if !skill_text.is_empty() {
                                            view! {
                                                <div class="mt-2 text-sm whitespace-pre-wrap" style="color: #374151;">
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
                                                <div class="mt-2 text-sm" style="background: #374151; color: white; padding: 8px; border-radius: 4px;">
                                                    <span class="font-semibold">"【ライフバースト】"</span>
                                                    {burst_text}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                </div>
                                <div class="ml-4 text-right flex flex-col gap-1" style="min-width: 120px;">
                                    {
                                        let level = card.level();
                                        if !level.is_empty() {
                                            view! {
                                                <div class="text-sm px-2 py-1 rounded" style="background: rgba(0,0,0,0.1); color: #374151;">
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
                                                <div class="text-sm px-2 py-1 rounded" style="background: rgba(0,0,0,0.1); color: #374151;">
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
                                                <div class="text-sm px-2 py-1 rounded" style="background: rgba(0,0,0,0.1); color: #374151;">
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
