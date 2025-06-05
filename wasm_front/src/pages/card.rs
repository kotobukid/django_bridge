use leptos::prelude::*;
use crate::types::ColorFilter;
use crate::components::{NavBar, ColorSelector, CardList};
use datapack::CardExport;
use std::collections::HashMap;

#[component]
pub fn CardPage() -> impl IntoView {
    let (color_filter, set_color_filter) = signal(ColorFilter::new());
    let (selected_features, set_selected_features) = signal(HashMap::<i32, bool>::new());
    let (filtered_cards, set_filtered_cards) = signal(Vec::<CardExport>::new());
    let (current_page, set_current_page) = signal(0usize);
    let cards_per_page = 50;

    // Load all cards from datapack
    let all_cards = Resource::new(
        || {},
        |_| async move {
            datapack::get_all_cards()
        }
    );

    // Apply filters when color or features change
    Effect::new(move || {
        if let Some(Ok(cards)) = all_cards.get() {
            let color = color_filter.get();
            let features = selected_features.get();
            
            let mut filtered = cards.clone();
            
            // Apply color filter
            if color.has_any() {
                let color_bits = color.to_bits();
                filtered = datapack::fetch_by_colors(&filtered, color_bits);
            }
            
            // Apply feature filters
            if !features.is_empty() {
                let active_features: Vec<_> = features.iter()
                    .filter(|(_, &v)| v)
                    .map(|(&k, _)| k)
                    .collect();
                    
                if !active_features.is_empty() {
                    filtered = datapack::fetch_by_features_and_native(&filtered, &active_features);
                }
            }
            
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
                selected_features=selected_features
                set_selected_features=set_selected_features
            />
            
            <div class="container mx-auto px-4 py-4">
                <div class="mb-6">
                    <ColorSelector
                        color_filter=color_filter
                        set_color_filter=set_color_filter
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
                                        <CardList cards=displayed_cards.get()/>
                                        
                                        // Pagination
                                        <div class="mt-8 flex justify-center gap-2">
                                            <button
                                                class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300"
                                                disabled=move || current_page.get() == 0
                                                on:click=move |_| set_current_page.update(|p| *p = p.saturating_sub(1))
                                            >
                                                "Previous"
                                            </button>
                                            
                                            <span class="px-4 py-2">
                                                {move || format!("Page {} of {}", current_page.get() + 1, total_pages.get())}
                                            </span>
                                            
                                            <button
                                                class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300"
                                                disabled=move || current_page.get() + 1 >= total_pages.get()
                                                on:click=move |_| set_current_page.update(|p| *p += 1)
                                            >
                                                "Next"
                                            </button>
                                        </div>
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