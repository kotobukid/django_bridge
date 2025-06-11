use leptos::*;
use leptos::prelude::*;
use crate::types::ProductFilter;

// 商品IDと名前のマッピング（静的データから生成）
fn get_product_list() -> Vec<(u8, &'static str, &'static str)> {
    datapack::gen::products::PRODUCT_LIST
        .iter()
        .filter(|(id, _, _)| {
            // カードデータに実際に存在する商品のみ表示
            datapack::gen::cards::CARD_LIST.iter().any(|card| card.18 == *id)
        })
        .copied()
        .collect()
}

#[component]
pub fn ProductSelector(
    product_filter: RwSignal<ProductFilter>,
) -> impl IntoView {
    let products = get_product_list();
    
    view! {
        <div class="product-selector">
            <h3 class="text-sm font-semibold mb-2">"商品"</h3>
            <div class="grid grid-cols-2 gap-1 max-h-64 overflow-y-auto">
                {products.into_iter().map(|(id, code, name)| {
                    let is_selected = move || product_filter.read().is_selected(id);
                    let display_name = format!("{} {}", code, name);
                    let title = display_name.clone();
                    view! {
                        <label class="flex items-center space-x-2 cursor-pointer p-1 hover:bg-gray-100">
                            <input
                                type="checkbox"
                                class="form-checkbox h-4 w-4 text-blue-600"
                                checked=is_selected
                                on:change=move |_| {
                                    product_filter.update(|f| f.toggle_product(id));
                                }
                            />
                            <span class="text-xs truncate" title=title>{display_name}</span>
                        </label>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}