use leptos::prelude::*;
use crate::types::ColorFilter;
use crate::components::{NavBar, ColorSelector, CardList};
use datapack::CardExport;
use std::collections::HashMap;

#[component]
pub fn CardPage() -> impl IntoView {
    let (color_filter, set_color_filter) = signal(ColorFilter::new());
    let (selected_features, set_selected_features) = signal(HashMap::<i32, bool>::new());
    let (selected_feature_names, set_selected_feature_names) = signal(Vec::<String>::new()); // 追加
    let (filtered_cards, set_filtered_cards) = signal(Vec::<CardExport>::new());
    let (current_page, set_current_page) = signal(0usize);
    let cards_per_page = 20;

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
            let feature_names = selected_feature_names.get();
            
            // 複合フィルタリングを使用
            let color_bits = if color.has_any() { color.to_bits() } else { 0 };
            let filtered = datapack::fetch_by_colors_and_features_native(&cards, color_bits, &feature_names);
            
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
                on_feature_change=set_selected_feature_names
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
                                        <Show when=move || !filtered_cards.get().is_empty()>
                                            <div class="mt-8 flex justify-center gap-2">
                                                <button
                                                    class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300"
                                                    prop:disabled=move || current_page.get() == 0
                                                    on:click=move |_| {
                                                        set_current_page.update(|p| {
                                                            if *p > 0 {
                                                                *p -= 1;
                                                            }
                                                        })
                                                    }
                                                >
                                                    "Previous"
                                                </button>
                                                
                                                <span class="px-4 py-2">
                                                    {move || {
                                                        let current = current_page.get();
                                                        let total = total_pages.get();
                                                        format!("Page {} of {}", current + 1, total.max(1))
                                                    }}
                                                </span>
                                                
                                                <button
                                                    class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300"
                                                    prop:disabled=move || {
                                                        let current = current_page.get();
                                                        let total = total_pages.get();
                                                        current + 1 >= total.max(1)
                                                    }
                                                    on:click=move |_| {
                                                        let total = total_pages.get();
                                                        set_current_page.update(|p| {
                                                            if *p + 1 < total {
                                                                *p += 1;
                                                            }
                                                        })
                                                    }
                                                >
                                                    "Next"
                                                </button>
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