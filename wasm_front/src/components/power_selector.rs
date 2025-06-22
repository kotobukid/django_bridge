use leptos::prelude::*;
use crate::types::PowerFilter;

#[component]
pub fn PowerSelector(
    power_filter: ReadSignal<PowerFilter>,
    set_power_filter: WriteSignal<PowerFilter>,
) -> impl IntoView {
    let thresholds = PowerFilter::threshold_options();
    
    // Clear button click handler
    let on_clear = move |_| {
        set_power_filter.update(|f| f.clear_all());
    };
    
    view! {
        <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-3">
            <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-medium text-gray-700">"パワー"</h3>
                <button
                    on:click=on_clear
                    class="text-xs text-gray-500 hover:text-gray-700 transition-colors"
                    class:invisible=move || !power_filter.get().has_any()
                >
                    "クリア"
                </button>
            </div>
            
            <div class="flex flex-wrap gap-1.5">
                <button
                    on:click=move |_| {
                        set_power_filter.update(|f| f.set_threshold(None));
                    }
                    class="px-2 py-1 text-xs rounded border transition-colors"
                    class:bg-blue-500=move || power_filter.get().min_power.is_none()
                    class:text-white=move || power_filter.get().min_power.is_none()
                    class:border-blue-500=move || power_filter.get().min_power.is_none()
                    class:bg-white=move || power_filter.get().min_power.is_some()
                    class:text-gray-700=move || power_filter.get().min_power.is_some()
                    class:border-gray-300=move || power_filter.get().min_power.is_some()
                    class:hover:bg-gray-50=move || power_filter.get().min_power.is_some()
                >
                    "すべて"
                </button>
                
                {thresholds.into_iter().map(|threshold| {
                    let is_selected = move || {
                        power_filter.get().min_power == Some(threshold)
                    };
                    
                    view! {
                        <button
                            on:click=move |_| {
                                set_power_filter.update(|f| {
                                    if f.min_power == Some(threshold) {
                                        f.set_threshold(None);
                                    } else {
                                        f.set_threshold(Some(threshold));
                                    }
                                });
                            }
                            class="px-2 py-1 text-xs rounded border transition-colors"
                            class:bg-blue-500=is_selected
                            class:text-white=is_selected
                            class:border-blue-500=is_selected
                            class:bg-white=move || !is_selected()
                            class:text-gray-700=move || !is_selected()
                            class:border-gray-300=move || !is_selected()
                            class:hover:bg-gray-50=move || !is_selected()
                            title=move || format!("パワー{}以上", threshold)
                        >
                            {PowerFilter::label_for_threshold(threshold)}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}