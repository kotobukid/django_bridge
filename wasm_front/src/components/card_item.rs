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

#[component]
pub fn Icons(
    #[prop(into)] colors: Vec<i32>
) -> impl IntoView {
    // Calculate width based on number of icons with 14px spacing (18px overlap from 32px)
    let width = if colors.is_empty() {
        0
    } else {
        32 + (colors.len() - 1) * 14
    };
    
    view! {
        <div class="color-icons" style=format!("width: {}px; flex-shrink: 0;", width)>
            {colors.into_iter().map(|color| {
                match color {
                    1 => view! { <IconWhite /> }.into_any(),
                    2 => view! { <IconBlue /> }.into_any(),
                    3 => view! { <IconRed /> }.into_any(),
                    4 => view! { <IconBlack /> }.into_any(),
                    5 => view! { <IconGreen /> }.into_any(),
                    6 => view! { <IconColorless /> }.into_any(),
                    _ => view! { <span></span> }.into_any(),
                }
            }).collect_view()}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Compact,
    Detailed,
}

#[component]
pub fn CardItem(
    card: CardExport,
) -> impl IntoView {
    // State for future interactive features
    let (view_mode, set_view_mode) = signal(ViewMode::Compact);
    
    // Calculate styles
    let color_style = datapack::bits_to_gradient_native(card.color() as i32);
    let card_url = format!("https://www.takaratomy.co.jp/products/wixoss/card_list.php?card=card_detail&card_no={}", card.code());
    
    // Determine border class based on card type
    let border_class = match card.card_type() {
        5 | 6 => "card-border-black", // Signi (5) or Spell (6)
        _ => "card-border-white",
    };
    
    view! {
        <div class=format!("card-item {}", border_class) style=format!("{}; border-radius: 8px; padding: 16px; margin: 8px 0;", color_style)>
            <div class="flex justify-between items-start">
                <div class="flex-1">
                    <h3 class="font-semibold text-lg" style="color: #374151;">
                        <a href=card_url target="_blank" class="flex items-center gap-1">
                            <Icons colors={get_colors_from_bits(card.color() as i32)} />
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
            </div>
        </div>
    }
}