use crate::types::ProductFilter;
use leptos::prelude::*;

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

#[derive(Debug, Clone)]
struct ProductCategory {
    name: String,
    display_name: String,
}

#[derive(Debug, Clone)]
struct ProductItem {
    id: u8,
    code: String,
    name: String,
}

#[component]
pub fn ProductOverlay(product_filter: RwSignal<ProductFilter>) -> impl IntoView {
    let products = get_product_list();

    // 商品タイプ別にグループ化
    let mut grouped_products = std::collections::HashMap::new();
    {
        let mut booster = Vec::new();
        let mut deck = Vec::new();
        let mut others = Vec::new();

        for (id, code, name) in products {
            let item = ProductItem {
                id,
                code: code.to_string(),
                name: name.to_string(),
            };
            
            if code.starts_with("WDA-F")    // キーセレクションの500円構築済みデッキ
                || code.starts_with("WDK-") // キーセレクションの構築済みデッキ
                || code.starts_with("WXD-") // オールスターの構築済みデッキ
                || code.starts_with("WXK-") // キーセレクションのブースター
                || code.starts_with("WXEX-") // オールスターの特別なブースター
                || code.starts_with("WX-") // オールスターの通常ブースター
            {
                // do nothing
            } else if code.contains("-P") || code.contains("-CP") {
                booster.push(item);
            } else if code.contains("-D") || code.contains("-CD") {
                deck.push(item);
            } else {
                others.push(item);
            }
        }

        grouped_products.insert("ブースターパック".to_string(), booster);
        grouped_products.insert("構築済みデッキ".to_string(), deck);
        grouped_products.insert("その他".to_string(), others);
    }

    let product_categories = vec![
        ProductCategory {
            name: "ブースターパック".to_string(),
            display_name: "ブースターパック".to_string(),
        },
        ProductCategory {
            name: "構築済みデッキ".to_string(),
            display_name: "構築済みデッキ".to_string(),
        },
        ProductCategory {
            name: "その他".to_string(),
            display_name: "その他".to_string(),
        },
    ];

    // 現在開いているカテゴリのID
    let (open_category, set_open_category) = signal::<Option<String>>(None);

    let toggle_category = move |category_name: String| {
        set_open_category.update(|current| {
            if current.as_ref() == Some(&category_name) {
                *current = None; // 既に開いている場合は閉じる
            } else {
                *current = Some(category_name); // 他のを閉じて開く
            }
        });
    };

    view! {
        <div class="space-y-4">
            {product_categories.into_iter().map(|category| {
                let category_id = category.name.clone();
                let category_id_for_toggle = category.name.clone();
                let category_id_for_open_check = category.name.clone();
                let category_id_for_products = category.name.clone();
                let grouped_products_clone = grouped_products.clone();
                let grouped_products_clone2 = grouped_products.clone();

                let is_open = Memo::new(move |_| {
                    open_category.get().as_ref() == Some(&category_id_for_open_check)
                });

                let has_selected_products = Memo::new(move |_| {
                    if let Some(products) = grouped_products_clone.get(&category_id_for_products) {
                        products.iter()
                            .any(|item| product_filter.read().is_selected(item.id))
                    } else {
                        false
                    }
                });

                view! {
                    <div class="border rounded-lg">
                        <button 
                            class={move || {
                                let base = "w-full px-4 py-3 text-left font-medium rounded-lg transition-colors";
                                let state_class = if has_selected_products.get() {
                                    "bg-blue-50 text-blue-700 border-blue-200"
                                } else if is_open.get() {
                                    "bg-gray-50 text-gray-700"
                                } else {
                                    "bg-white text-gray-700 hover:bg-gray-50"
                                };
                                format!("{} {}", base, state_class)
                            }}
                            on:click=move |_| toggle_category(category_id_for_toggle.clone())
                        >
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <Show when=move || has_selected_products.get()>
                                        <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
                                    </Show>
                                    <span>{category.display_name}</span>
                                </div>
                                <span class={move || if is_open.get() { "transform rotate-180" } else { "" }}>
                                    "▼"
                                </span>
                            </div>
                        </button>

                        <Show when=move || is_open.get()>
                            <div class="border-t bg-gray-50">
                                <div class="p-3 space-y-2">
                                    {if let Some(products) = grouped_products_clone2.get(&category_id) {
                                        if products.is_empty() {
                                            view! { <div class="text-gray-500 text-sm p-2">"No products"</div> }.into_any()
                                        } else {
                                            products.iter().map(|item| {
                                                let item_id = item.id;
                                                let is_selected = Signal::derive(move || {
                                                    product_filter.read().is_selected(item_id)
                                                });
                                                let display_name = item.name.clone();
                                                let code_display = item.code.clone();

                                                view! {
                                                    <label class="flex items-start space-x-3 p-2 rounded hover:bg-white cursor-pointer transition-colors">
                                                        <input
                                                            type="checkbox"
                                                            class="form-checkbox h-4 w-4 text-blue-600 mt-1 flex-shrink-0"
                                                            checked=is_selected
                                                            on:change=move |_| {
                                                                product_filter.update(|f| f.toggle_product(item_id));
                                                            }
                                                        />
                                                        <div class="flex-1 min-w-0">
                                                            <div class="font-medium text-sm text-blue-600">{code_display}</div>
                                                            <div class="text-xs text-gray-600 mt-1 line-clamp-2">{display_name}</div>
                                                        </div>
                                                    </label>
                                                }
                                            }).collect_view().into_any()
                                        }
                                    } else {
                                        view! { <div class="text-gray-500 text-sm p-2">"No products"</div> }.into_any()
                                    }}
                                </div>
                            </div>
                        </Show>
                    </div>
                }.into_any()
            }).collect::<Vec<_>>()}
        </div>
    }
}