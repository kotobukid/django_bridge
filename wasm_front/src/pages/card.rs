use crate::components::{CardList, CardTypeSelector, ColorSelector, FeatureOverlay, OverlayButton, Pagination, ProductOverlay};
use crate::types::{CardTypeFilter, ColorFilter, ProductFilter};
use datapack::CardExport;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn CardPage() -> impl IntoView {
    let (color_filter, set_color_filter) = signal(ColorFilter::new());
    let (card_type_filter, set_card_type_filter) = signal(CardTypeFilter::new());
    let product_filter = RwSignal::new(ProductFilter::new());
    let selected_features = RwSignal::new(HashMap::<String, bool>::new());
    let (selected_feature_names, set_selected_feature_names) = signal(Vec::<String>::new());
    let (filtered_cards, set_filtered_cards) = signal(Vec::<CardExport>::new());
    let (current_page, set_current_page) = signal(0usize);
    let cards_per_page = 20;
    
    // オーバーレイ表示状態
    let (show_feature_overlay, set_show_feature_overlay) = signal(false);
    let (show_product_overlay, set_show_product_overlay) = signal(false);
    


    // Load all cards from datapack
    let all_cards = Resource::new(|| {}, |_| async move { datapack::get_all_cards() });

    // Apply filters when color, features, card types, or products change
    Effect::new(move || {
        if let Some(Ok(cards)) = all_cards.get() {
            let color = color_filter.get();
            let card_type = card_type_filter.get();
            let product = product_filter.read();
            let feature_names = selected_feature_names.get();

            // 複合フィルタリングを使用
            let color_bits = if color.has_any() { color.to_bits() } else { 0 };
            let card_types = if card_type.has_any() { 
                card_type.get_selected_card_types() 
            } else { 
                Vec::new() 
            };
            let products = if product.has_any() {
                product.selected_products.clone()
            } else {
                Vec::new()
            };
            
            let filtered = datapack::fetch_by_colors_features_card_types_and_products_native(
                &cards, 
                color_bits, 
                &feature_names,
                &card_types,
                &products
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

    // フィルタの有効状態を判定
    let has_active_features = Memo::new(move |_| {
        !selected_features.read().is_empty()
    });
    
    let has_active_products = Memo::new(move |_| {
        product_filter.read().has_any()
    });

    view! {
        <div class="min-h-screen bg-gray-100">
            <div class="container mx-auto px-4 py-4">
                // フィルタボタン行
                <div class="mb-6 space-y-4">
                    <div class="flex flex-wrap gap-3">
                        <OverlayButton
                            label="カード効果".to_string()
                            is_active=Signal::derive(move || has_active_features.get())
                            on_click=Callback::new(move |_| set_show_feature_overlay.set(true))
                        />
                        <OverlayButton
                            label="製品".to_string()
                            is_active=Signal::derive(move || has_active_products.get())
                            on_click=Callback::new(move |_| set_show_product_overlay.set(true))
                        />
                    </div>
                    
                    // 常時表示フィルタ
                    <div class="space-y-3">
                        <ColorSelector
                            color_filter=color_filter
                            set_color_filter=set_color_filter
                        />
                        <CardTypeSelector
                            card_type_filter=card_type_filter
                            set_card_type_filter=set_card_type_filter
                        />
                    </div>
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
            
            // フィーチャーオーバーレイ
            <Show when=move || show_feature_overlay.get()>
                <div 
                    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-85"
                    on:click=move |_| set_show_feature_overlay.set(false)
                >
                    <div 
                        class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 overflow-hidden"
                        on:click=|e| e.stop_propagation()
                    >
                        <div class="flex items-center justify-between p-4 border-b">
                            <h2 class="text-lg font-semibold">"カード効果選択"</h2>
                            <button 
                                class="text-gray-500 hover:text-gray-700 text-xl font-bold px-2"
                                on:click=move |_| set_show_feature_overlay.set(false)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="p-4 overflow-y-auto max-h-[60vh]">
                            <FeatureOverlay
                                selected_features=selected_features
                                on_feature_change=set_selected_feature_names
                            />
                        </div>
                    </div>
                </div>
            </Show>
            
            // 商品オーバーレイ
            <Show when=move || show_product_overlay.get()>
                <div 
                    class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-85"
                    on:click=move |_| set_show_product_overlay.set(false)
                >
                    <div 
                        class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 overflow-hidden"
                        on:click=|e| e.stop_propagation()
                    >
                        <div class="flex items-center justify-between p-4 border-b">
                            <h2 class="text-lg font-semibold">"製品選択"</h2>
                            <button 
                                class="text-gray-500 hover:text-gray-700 text-xl font-bold px-2"
                                on:click=move |_| set_show_product_overlay.set(false)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="p-4 overflow-y-auto max-h-[60vh]">
                            <ProductOverlay
                                product_filter=product_filter
                            />
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
