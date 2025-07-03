use crate::types::ProductFilter;
use leptos::prelude::*;

// 商品の種類を判定する関数
fn get_product_category(code: &str) -> &'static str {
    if code.starts_with("WX") && (code.contains("-D") || code.starts_with("WXDi-D")) {
        "構築済みデッキ"
    } else if code.starts_with("WX")
        && (code.contains("-P") || code.starts_with("WXDi-P") || code.contains("-CP"))
    {
        "ブースターパック"
    } else {
        "その他"
    }
}

// 商品IDと名前のマッピング（カテゴリ別に分類）
fn get_categorized_product_list() -> Vec<(&'static str, Vec<(u8, &'static str, &'static str)>)> {
    leptos::logging::log!(
        "Total products in PRODUCT_LIST: {}",
        datapack::gen::products::PRODUCT_LIST.len()
    );
    leptos::logging::log!(
        "Total cards in CARD_LIST: {}",
        datapack::gen::cards::CARD_LIST.len()
    );

    let all_products: Vec<(u8, &'static str, &'static str)> = datapack::gen::products::PRODUCT_LIST
        .iter()
        .filter(|(id, _, _)| {
            // カードデータに実際に存在する商品のみ表示（正しい位置17を使用）
            let has_cards = datapack::gen::cards::CARD_LIST
                .iter()
                .any(|card| card.17 == *id);
            if has_cards {
                leptos::logging::log!("Product {} has cards", id);
            }
            has_cards
        })
        .copied()
        .collect();

    leptos::logging::log!("Filtered products count: {}", all_products.len());

    let mut booster_packs = Vec::new();
    let mut constructed_decks = Vec::new();
    let mut others = Vec::new();

    for product in all_products {
        let (_id, code, _name) = product;
        let category = get_product_category(code);
        leptos::logging::log!("Product {} ({}) -> category: {}", _id, code, category);
        match category {
            "ブースターパック" => booster_packs.push(product),
            "構築済みデッキ" => constructed_decks.push(product),
            _ => others.push(product),
        }
    }

    // 各カテゴリを商品コード順でソート
    booster_packs.sort_by(|a, b| a.1.cmp(b.1));
    constructed_decks.sort_by(|a, b| a.1.cmp(b.1));
    others.sort_by(|a, b| a.1.cmp(b.1));

    leptos::logging::log!("Booster packs: {}", booster_packs.len());
    leptos::logging::log!("Constructed decks: {}", constructed_decks.len());
    leptos::logging::log!("Others: {}", others.len());

    let mut categories = Vec::new();
    if !booster_packs.is_empty() {
        categories.push(("ブースターパック", booster_packs));
    }
    if !constructed_decks.is_empty() {
        categories.push(("構築済みデッキ", constructed_decks));
    }
    if !others.is_empty() {
        categories.push(("その他", others));
    }

    categories
}

#[component]
fn ProductCategorySection(
    category_name: &'static str,
    products: Vec<(u8, &'static str, &'static str)>,
    product_filter: RwSignal<ProductFilter>,
) -> impl IntoView {
    leptos::logging::log!(
        "ProductCategorySection rendering: {} with {} products",
        category_name,
        products.len()
    );
    if products.is_empty() {
        leptos::logging::log!("EMPTY PRODUCTS for category: {}", category_name);
    } else {
        for (id, code, name) in &products {
            leptos::logging::log!("  Product: {} {} {}", id, code, name);
        }
    }

    view! {
        <div class="category-section">
            <h4 class="text-xs font-medium text-gray-700 mb-1">{category_name}</h4>
            <div class="space-y-1">
                {if products.is_empty() {
                    vec![view! {
                        <div class="text-xs text-gray-500 ml-2">"No products"</div>
                    }.into_any()]
                } else {
                    products.into_iter().map(|(id, code, name)| {
                        let is_selected = move || product_filter.read().is_selected(id);
                        let display_name = format!("{code} {name}");
                        let title = display_name.clone();
                        view! {
                            <label class="flex items-center space-x-2 cursor-pointer p-1 hover:bg-gray-100 ml-2">
                                <input
                                    type="checkbox"
                                    class="form-checkbox h-3 w-3 text-blue-600"
                                    checked=is_selected
                                    on:change=move |_| {
                                        product_filter.update(|f| f.toggle_product(id));
                                    }
                                />
                                <span class="text-xs truncate" title=title>{display_name}</span>
                            </label>
                        }.into_any()
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[allow(dead_code)]
#[component]
pub fn ProductSelector(product_filter: RwSignal<ProductFilter>) -> impl IntoView {
    leptos::logging::log!("ProductSelector component rendering...");
    let categorized_products = get_categorized_product_list();
    leptos::logging::log!("Got {} categories", categorized_products.len());

    view! {
        <div class="product-selector">
            <h3 class="text-sm font-semibold mb-2">"商品"</h3>
            <div class="max-h-64 overflow-y-auto space-y-3">
                {categorized_products.into_iter().map(|(category_name, products)| {
                    leptos::logging::log!("Rendering category: {} with {} products", category_name, products.len());
                    view! {
                        <ProductCategorySection
                            category_name=category_name
                            products=products
                            product_filter=product_filter
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
