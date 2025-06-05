use leptos::prelude::*;
use crate::types::get_feature_categories;
use std::collections::HashMap;

#[component]
pub fn NavBar(
    selected_features: ReadSignal<HashMap<i32, bool>>,
    set_selected_features: WriteSignal<HashMap<i32, bool>>,
) -> impl IntoView {
    let (dropdown_open, set_dropdown_open) = signal(false);
    
    let toggle_feature = move |feature_id: i32| {
        set_selected_features.update(|features| {
            if features.contains_key(&feature_id) {
                features.remove(&feature_id);
            } else {
                features.insert(feature_id, true);
            }
        });
    };
    
    let clear_all = move |_| {
        set_selected_features.set(HashMap::new());
    };
    
    let selected_count = Memo::new(move |_| {
        selected_features.get().len()
    });

    view! {
        <nav class="bg-gray-800 text-white">
            <div class="container mx-auto px-4">
                <div class="flex items-center justify-between h-16">
                    <div class="text-xl font-bold">
                        "WIXOSS Card Filter"
                    </div>
                    
                    <div class="relative">
                        <button
                            class="px-4 py-2 bg-gray-700 rounded hover:bg-gray-600 transition-colors flex items-center gap-2"
                            on:click=move |_| set_dropdown_open.update(|v| *v = !*v)
                        >
                            "Features"
                            {move || if selected_count.get() != 0 {
                                format!(" ({})", selected_count.get())
                            } else {
                                String::new()
                            }}
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                            </svg>
                        </button>
                        
                        <Show when=move || dropdown_open.get()>
                            <div class="absolute right-0 mt-2 w-64 bg-white rounded-lg shadow-lg z-50 text-gray-800">
                                <div class="p-4">
                                    {get_feature_categories().into_iter().map(|(category, features)| {
                                        view! {
                                            <div class="mb-4">
                                                <h3 class="font-semibold mb-2">{category}</h3>
                                                <div class="space-y-1">
                                                    {features.into_iter().map(|feature| {
                                                        let feature_id = feature.id;
                                                        let is_selected = Memo::new(move |_| {
                                                            selected_features.get().contains_key(&feature_id)
                                                        });
                                                        
                                                        view! {
                                                            <label class="flex items-center gap-2 cursor-pointer hover:bg-gray-100 p-1 rounded">
                                                                <input
                                                                    type="checkbox"
                                                                    class="feature-checkbox"
                                                                    checked=is_selected
                                                                    on:change=move |_| toggle_feature(feature_id)
                                                                />
                                                                <span class="text-sm">{feature.name}</span>
                                                            </label>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        }
                                    }).collect_view()}
                                    
                                    <Show when=move || selected_count.get() != 0>
                                        <button
                                            class="mt-4 w-full px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
                                            on:click=clear_all
                                        >
                                            "Clear All"
                                        </button>
                                    </Show>
                                </div>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}