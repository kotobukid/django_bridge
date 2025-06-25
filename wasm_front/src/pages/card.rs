use crate::components::{
    BurstFeatureOverlay, CardList, CardTypeSelector, ClearAllButton, ColorSelector, FeatureOverlay,
    FeatureShortcuts, KlassOverlay, LevelSelector, OverlayButton, Pagination, PowerSelector,
    ProductOverlay, ScrollToTopButton, TextSearch,
};
use crate::types::{
    CardTypeFilter, ColorFilter, KlassFilter, LBFilter, LevelFilter, PowerFilter, ProductFilter,
};
use datapack::CardExport;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn CardPage() -> impl IntoView {
    let (color_filter, set_color_filter) = signal(ColorFilter::new());
    let (card_type_filter, set_card_type_filter) = signal(CardTypeFilter::new());
    let (level_filter, set_level_filter) = signal(LevelFilter::new());
    let (power_filter, set_power_filter) = signal(PowerFilter::new());
    let (lb_filter, set_lb_filter) = signal(LBFilter::new());
    let (search_text, set_search_text) = signal(String::new());
    let product_filter = RwSignal::new(ProductFilter::new());
    let klass_filter = RwSignal::new(KlassFilter::new());
    let selected_features = RwSignal::new(HashMap::<String, bool>::new());
    let (selected_feature_names, set_selected_feature_names) = signal(Vec::<String>::new());
    let selected_burst_features = RwSignal::new(HashMap::<String, bool>::new());
    let (selected_burst_feature_names, set_selected_burst_feature_names) =
        signal(Vec::<String>::new());
    let (filtered_cards, set_filtered_cards) = signal(Vec::<CardExport>::new());
    let (current_page, set_current_page) = signal(0usize);
    let cards_per_page = 20;

    // オーバーレイ表示状態
    let (show_feature_overlay, set_show_feature_overlay) = signal(false);
    let (show_product_overlay, set_show_product_overlay) = signal(false);
    let (show_klass_overlay, set_show_klass_overlay) = signal(false);
    let (show_burst_feature_overlay, set_show_burst_feature_overlay) = signal(false);

    // オーバーレイの強制再描画用
    let (feature_overlay_key, set_feature_overlay_key) = signal(0u32);
    let (product_overlay_key, set_product_overlay_key) = signal(0u32);

    // Load all cards from datapack
    let all_cards = Resource::new(|| {}, |_| async move { datapack::get_all_cards() });

    // Apply filters when color, features, card types, products, levels, klass, or text search change
    Effect::new(move || {
        if let Some(Ok(cards)) = all_cards.get() {
            let color = color_filter.get();
            let card_type = card_type_filter.get();
            let level = level_filter.get();
            let power = power_filter.get();
            let lb = lb_filter.get();
            let product = product_filter.read();
            let klass = klass_filter.read();
            let feature_names = selected_feature_names.get();
            let burst_feature_names = selected_burst_feature_names.get();
            let text_search = search_text.get();

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

            let levels = if level.has_any() {
                level.selected_levels.clone()
            } else {
                Vec::new()
            };

            let klass_bits = klass.selected_bits;

            let mut filtered = datapack::fetch_by_colors_features_card_types_products_levels_power_threshold_klass_and_text_native(
                &cards,
                color_bits,
                &feature_names,
                &card_types,
                &products,
                &levels,
                power.min_power,
                klass_bits,
                &text_search,
            );

            // BurstFeatureフィルタリングを追加適用
            if !burst_feature_names.is_empty() {
                filtered =
                    datapack::fetch_by_burst_features_and_native(&filtered, &burst_feature_names);
            }

            // LBフィルタリングを追加適用
            if lb.has_any() {
                filtered.retain(|card| {
                    match lb.selection {
                        1 => card.has_burst() == 1, // LBあり
                        2 => card.has_burst() == 2, // LBなし
                        _ => true,
                    }
                });
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
        total.div_ceil(cards_per_page)
    });

    // フィルタの有効状態を判定
    let has_active_features = Memo::new(move |_| !selected_features.read().is_empty());
    let has_active_burst_features = Memo::new(move |_| !selected_burst_features.read().is_empty());

    let has_active_products = Memo::new(move |_| product_filter.read().has_any());

    let has_active_klass = Memo::new(move |_| klass_filter.read().has_any());

    // いずれかのフィルタがアクティブかどうかを判定
    let has_any_active_filter = Memo::new(move |_| {
        color_filter.get().has_any()
            || card_type_filter.get().has_any()
            || level_filter.get().has_any()
            || power_filter.get().has_any()
            || lb_filter.get().has_any()
            || !search_text.get().is_empty()
            || has_active_features.get()
            || has_active_burst_features.get()
            || has_active_products.get()
            || has_active_klass.get()
    });

    // 全クリア処理
    let clear_all_filters = move |_| {
        set_search_text.set(String::new());
        set_color_filter.set(ColorFilter::new());
        set_card_type_filter.set(CardTypeFilter::new());
        set_level_filter.set(LevelFilter::new());
        set_power_filter.set(PowerFilter::new());
        set_lb_filter.set(LBFilter::new());
        selected_features.update(|f| f.clear());
        set_selected_feature_names.set(Vec::new());
        selected_burst_features.update(|f| f.clear());
        set_selected_burst_feature_names.set(Vec::new());
        product_filter.update(|f| f.clear_all());
        klass_filter.update(|f| f.clear_all());
    };

    view! {
        <div class="min-h-screen bg-gray-100">
            <div class="container mx-auto px-4 py-4">
                // フィルタボタン行
                <div class="mb-6 space-y-4">
                    <div class="flex flex-wrap gap-3">
                        <ClearAllButton
                            is_active=Signal::derive(move || has_any_active_filter.get())
                            on_click=Callback::new(clear_all_filters)
                        />
                        <OverlayButton
                            label="カード効果".to_string()
                            is_active=Signal::derive(move || has_active_features.get())
                            on_click=Callback::new(move |_| set_show_feature_overlay.set(true))
                        />
                        <OverlayButton
                            label="LB効果".to_string()
                            is_active=Signal::derive(move || has_active_burst_features.get())
                            on_click=Callback::new(move |_| set_show_burst_feature_overlay.set(true))
                        />
                        <OverlayButton
                            label="製品".to_string()
                            is_active=Signal::derive(move || has_active_products.get())
                            on_click=Callback::new(move |_| set_show_product_overlay.set(true))
                        />
                        <OverlayButton
                            label="クラス".to_string()
                            is_active=Signal::derive(move || has_active_klass.get())
                            on_click=Callback::new(move |_| set_show_klass_overlay.set(true))
                        />
                    </div>

                    // 常時表示フィルタ
                    <div class="space-y-3">
                        <TextSearch
                            search_text=search_text
                            set_search_text=set_search_text
                        />

                        // Color and Feature shortcuts in responsive grid
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <ColorSelector
                                color_filter=color_filter
                                set_color_filter=set_color_filter
                            />
                            <FeatureShortcuts
                                selected_features=selected_features
                                on_feature_change=set_selected_feature_names
                            />
                        </div>

                        <LevelSelector
                            level_filter=level_filter
                            set_level_filter=set_level_filter
                        />
                        <PowerSelector
                            power_filter=power_filter
                            set_power_filter=set_power_filter
                        />
                        <CardTypeSelector
                            card_type_filter=card_type_filter
                            set_card_type_filter=set_card_type_filter
                        />
                    </div>
                </div>


                <Suspense fallback=move || view! { <div>"Loading cards..."</div> }>
                    {move || {
                        if let Some(result) = all_cards.get() {
                            match result {
                                Ok(_) => view! {
                                    <div>
                                        // Top Pagination
                                        <Show when=move || !filtered_cards.get().is_empty()>
                                            <div class="mb-4" data-pagination-top="true">
                                                <Pagination
                                                    current_page=current_page
                                                    total_pages=total_pages
                                                    set_current_page=set_current_page
                                                />
                                            </div>
                                        </Show>

                                        <CardList cards=displayed_cards.get() total_count=filtered_cards.get().len()/>

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
                    class="fixed inset-0 z-50 flex items-center justify-center"
                    style="background-color: rgba(0, 0, 0, 0.75);"
                    on:click=move |_| set_show_feature_overlay.set(false)
                >
                    <div
                        class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 overflow-hidden"
                        on:click=|e| e.stop_propagation()
                    >
                        <div class="flex items-center justify-between p-4 border-b">
                            <div class="flex items-center space-x-3">
                                <h2 class="text-lg font-semibold">"カード効果選択"</h2>
                                <Show when=move || has_active_features.get()>
                                    <button
                                        class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition-colors"
                                        on:click=move |_| {
                                            selected_features.update(|f| f.clear());
                                            set_selected_feature_names.set(Vec::new());
                                            // オーバーレイを強制再描画
                                            set_feature_overlay_key.update(|k| *k += 1);
                                        }
                                    >
                                        "クリア"
                                    </button>
                                </Show>
                            </div>
                            <button
                                class="text-gray-500 hover:text-gray-700 text-xl font-bold px-2"
                                on:click=move |_| set_show_feature_overlay.set(false)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="p-4 overflow-y-auto max-h-[60vh]">
                            {move || {
                                let _key = feature_overlay_key.get(); // キーを監視
                                view! {
                                    <FeatureOverlay
                                        selected_features=selected_features
                                        on_feature_change=set_selected_feature_names
                                    />
                                }
                            }}
                        </div>
                    </div>
                </div>
            </Show>

            // LB効果オーバーレイ
            <Show when=move || show_burst_feature_overlay.get()>
                <div
                    class="fixed inset-0 z-50 flex items-center justify-center"
                    style="background-color: rgba(0, 0, 0, 0.75);"
                    on:click=move |_| set_show_burst_feature_overlay.set(false)
                >
                    <div
                        class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 overflow-hidden"
                        on:click=|e| e.stop_propagation()
                    >
                        <div class="flex items-center justify-between p-4 border-b">
                            <div class="flex items-center space-x-3">
                                <h2 class="text-lg font-semibold">"ライフバースト効果選択"</h2>
                                <Show when=move || has_active_burst_features.get()>
                                    <button
                                        class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition-colors"
                                        on:click=move |_| {
                                            selected_burst_features.update(|f| f.clear());
                                            set_selected_burst_feature_names.set(Vec::new());
                                        }
                                    >
                                        "クリア"
                                    </button>
                                </Show>
                            </div>
                            <button
                                class="text-gray-500 hover:text-gray-700 text-xl font-bold px-2"
                                on:click=move |_| set_show_burst_feature_overlay.set(false)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="overflow-y-auto max-h-[60vh]">
                            <BurstFeatureOverlay
                                selected_burst_features=selected_burst_features
                                on_burst_feature_change=set_selected_burst_feature_names
                                lb_filter=lb_filter
                                set_lb_filter=set_lb_filter
                            />
                        </div>
                    </div>
                </div>
            </Show>

            // 商品オーバーレイ
            <Show when=move || show_product_overlay.get()>
                <div
                    class="fixed inset-0 z-50 flex items-center justify-center"
                    style="background-color: rgba(0, 0, 0, 0.75);"
                    on:click=move |_| set_show_product_overlay.set(false)
                >
                    <div
                        class="bg-white rounded-lg shadow-lg max-w-4xl max-h-[80vh] w-full mx-4 overflow-hidden"
                        on:click=|e| e.stop_propagation()
                    >
                        <div class="flex items-center justify-between p-4 border-b">
                            <div class="flex items-center space-x-3">
                                <h2 class="text-lg font-semibold">"製品選択"</h2>
                                <Show when=move || has_active_products.get()>
                                    <button
                                        class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md transition-colors"
                                        on:click=move |_| {
                                            product_filter.update(|f| f.clear_all());
                                            // オーバーレイを強制再描画
                                            set_product_overlay_key.update(|k| *k += 1);
                                        }
                                    >
                                        "クリア"
                                    </button>
                                </Show>
                            </div>
                            <button
                                class="text-gray-500 hover:text-gray-700 text-xl font-bold px-2"
                                on:click=move |_| set_show_product_overlay.set(false)
                            >
                                "×"
                            </button>
                        </div>
                        <div class="p-4 overflow-y-auto max-h-[60vh]">
                            {move || {
                                let _key = product_overlay_key.get(); // キーを監視
                                view! {
                                    <ProductOverlay
                                        product_filter=product_filter
                                    />
                                }
                            }}
                        </div>
                    </div>
                </div>
            </Show>

            // Klassオーバーレイ
            <KlassOverlay
                klass_filter=klass_filter
                is_open=show_klass_overlay
                on_close=set_show_klass_overlay
            />

            // スクロールトップボタン
            <ScrollToTopButton />
        </div>
    }
}
