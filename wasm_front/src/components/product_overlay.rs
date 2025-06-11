use crate::types::ProductFilter;
use leptos::prelude::*;
use leptos::*;

// 商品IDと名前のマッピング（静的データから生成）
fn get_product_list() -> Vec<(u8, &'static str, &'static str)> {
    datapack::gen::products::PRODUCT_LIST
        .iter()
        .filter(|(id, _, _)| {
            // カードデータに実際に存在する商品のみ表示
            datapack::gen::cards::CARD_LIST
                .iter()
                .any(|card| card.18 == *id)
        })
        .copied()
        .collect()
}

#[component]
pub fn ProductOverlay(product_filter: RwSignal<ProductFilter>) -> impl IntoView {
    let products = get_product_list();

    // 商品タイプ別にグループ化
    let grouped_products = {
        let mut booster = Vec::new();
        let mut deck = Vec::new();
        let mut others = Vec::new();

        for (id, code, name) in products {
            if code.contains("-P") || code.contains("-CP") {
                booster.push((id, code, name));
            } else if code.contains("-D") || code.contains("-CD") {
                deck.push((id, code, name));
            } else {
                others.push((id, code, name));
            }
        }

        vec![
            ("ブースターパック", booster),
            ("構築済みデッキ", deck),
            ("その他", others),
        ]
    };

    view! {
        <div class="space-y-6">
            {grouped_products.into_iter().map(|(category, products)| {
                if products.is_empty() {
                    view! { <div></div> }.into_any()
                } else {
                    view! {
                        <div>
                            <h3 class="text-lg font-semibold mb-3 text-gray-800">{category}</h3>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                                {products.into_iter().map(|(id, code, name)| {
                                    let is_selected = move || product_filter.read().is_selected(id);
                                    let display_name = format!("{}", name);
                                    let code_display = code.to_string();

                                    view! {
                                        <label class="flex items-start space-x-3 p-3 rounded-lg border hover:bg-gray-50 cursor-pointer transition-colors">
                                            <input
                                                type="checkbox"
                                                class="form-checkbox h-4 w-4 text-blue-600 mt-1 flex-shrink-0"
                                                checked=is_selected
                                                on:change=move |_| {
                                                    product_filter.update(|f| f.toggle_product(id));
                                                }
                                            />
                                            <div class="flex-1 min-w-0">
                                                <div class="font-medium text-sm text-blue-600">{code_display}</div>
                                                <div class="text-xs text-gray-600 mt-1 line-clamp-2">{display_name}</div>
                                            </div>
                                        </label>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_any()
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
