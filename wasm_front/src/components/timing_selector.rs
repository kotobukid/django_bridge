use crate::types::TimingFilter;
use leptos::prelude::*;

#[component]
pub fn TimingSelector(
    timing_filter: ReadSignal<TimingFilter>,
    set_timing_filter: WriteSignal<TimingFilter>,
) -> impl IntoView {
    view! {
        <div class="bg-white p-4 rounded-lg shadow-sm border">
            <div class="flex flex-wrap gap-2">
                <button
                    class=move || {
                        if timing_filter.get().main_phase {
                            "px-3 py-1.5 text-sm rounded-md bg-blue-600 text-white border border-blue-600 hover:bg-blue-700 transition-colors"
                        } else {
                            "px-3 py-1.5 text-sm rounded-md bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 transition-colors"
                        }
                    }
                    on:click=move |_| {
                        set_timing_filter.update(|f| f.main_phase = !f.main_phase);
                    }
                >
                    "メインフェイズ"
                </button>

                <button
                    class=move || {
                        if timing_filter.get().attack_phase {
                            "px-3 py-1.5 text-sm rounded-md bg-blue-600 text-white border border-blue-600 hover:bg-blue-700 transition-colors"
                        } else {
                            "px-3 py-1.5 text-sm rounded-md bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 transition-colors"
                        }
                    }
                    on:click=move |_| {
                        set_timing_filter.update(|f| f.attack_phase = !f.attack_phase);
                    }
                >
                    "アタックフェイズ"
                </button>

                <button
                    class=move || {
                        if timing_filter.get().spell_cutins {
                            "px-3 py-1.5 text-sm rounded-md bg-blue-600 text-white border border-blue-600 hover:bg-blue-700 transition-colors"
                        } else {
                            "px-3 py-1.5 text-sm rounded-md bg-white text-gray-700 border border-gray-300 hover:bg-gray-50 transition-colors"
                        }
                    }
                    on:click=move |_| {
                        set_timing_filter.update(|f| f.spell_cutins = !f.spell_cutins);
                    }
                >
                    "スペルカットイン"
                </button>
            </div>
        </div>
    }
}
