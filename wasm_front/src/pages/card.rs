use crate::components::{CardList, CardTypeSelector, ColorSelector, NavBar, Pagination};
use crate::types::{CardTypeFilter, ColorFilter};
use datapack::CardExport;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn CardPage() -> impl IntoView {
    let (color_filter, set_color_filter) = signal(ColorFilter::new());
    let (card_type_filter, set_card_type_filter) = signal(CardTypeFilter::new());
    let (selected_features, set_selected_features) = signal(HashMap::<i32, bool>::new());
    let (selected_feature_names, set_selected_feature_names) = signal(Vec::<String>::new()); // 追加
    let (filtered_cards, set_filtered_cards) = signal(Vec::<CardExport>::new());
    let (current_page, set_current_page) = signal(0usize);
    let cards_per_page = 20;
    


    // Load all cards from datapack
    let all_cards = Resource::new(|| {}, |_| async move { datapack::get_all_cards() });

    // Apply filters when color, features, or card types change
    Effect::new(move || {
        if let Some(Ok(cards)) = all_cards.get() {
            let color = color_filter.get();
            let card_type = card_type_filter.get();
            let feature_names = selected_feature_names.get();

            // 複合フィルタリングを使用
            let color_bits = if color.has_any() { color.to_bits() } else { 0 };
            let card_types = if card_type.has_any() { 
                card_type.get_selected_card_types() 
            } else { 
                Vec::new() 
            };
            
            let filtered = datapack::fetch_by_colors_features_and_card_types_native(
                &cards, 
                color_bits, 
                &feature_names,
                &card_types
            );

            set_filtered_cards.set(filtered);
            set_current_page.set(0);
        }
    });

    let displayed_cards = Memo::new(move |_| {
        let cards = filtered_cards.get();
        let page = current_page.get();
        let start = page * cards_per_page;
        let end = (start + cards_per_page).min(cards.len());

        cards[start..end].to_vec()
    });

    let total_pages = Memo::new(move |_| {
        let total = filtered_cards.get().len();
        (total + cards_per_page - 1) / cards_per_page
    });

    view! {
        <div class="min-h-screen bg-gray-100">
            <NavBar
                _selected_features=selected_features
                _set_selected_features=set_selected_features
                on_feature_change=set_selected_feature_names
            />

            <div class="container mx-auto px-4 py-4">
                <div class="mb-6 space-y-4">
                    <ColorSelector
                        color_filter=color_filter
                        set_color_filter=set_color_filter
                    />
                    <CardTypeSelector
                        card_type_filter=card_type_filter
                        set_card_type_filter=set_card_type_filter
                    />
                </div>

                <div class="mb-4 text-sm text-gray-600">
                    {move || format!("Found {} cards", filtered_cards.get().len())}
                </div>

                <Suspense fallback=move || view! { <div>"Loading cards..."</div> }>
                    {move || {
                        if let Some(result) = all_cards.get() {
                            match result {
                                Ok(_) => view! {
                                    <div>
                                        // Top Pagination
                                        <Show when=move || !filtered_cards.get().is_empty()>
                                            <div class="mb-4">
                                                <Pagination
                                                    current_page=current_page
                                                    total_pages=total_pages
                                                    set_current_page=set_current_page
                                                />
                                            </div>
                                        </Show>

                                        <CardList cards=displayed_cards.get()/>

                                        // Bottom Pagination
                                        <Show when=move || !filtered_cards.get().is_empty()>
                                            <div class="mt-8">
                                                <Pagination
                                                    current_page=current_page
                                                    total_pages=total_pages
                                                    set_current_page=set_current_page
                                                />
                                            </div>
                                        </Show>
                                    </div>
                                }.into_any(),
                                Err(e) => view! {
                                    <div class="text-red-600">
                                        {format!("Error loading cards: {:?}", e)}
                                    </div>
                                }.into_any()
                            }
                        } else {
                            view! { <div>"Loading..."</div> }.into_any()
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}
