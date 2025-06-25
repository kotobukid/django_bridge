use leptos::prelude::*;
use leptos::*;

#[component]
pub fn OverlayButton(
    #[prop(into)] label: String,
    #[prop(into)] is_active: Signal<bool>,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class={move || {
                let base = "px-4 py-2 rounded-lg border transition-colors duration-200";
                if is_active.get() {
                    format!("{} bg-blue-100 border-blue-500 text-blue-700 font-medium", base)
                } else {
                    format!("{} bg-white border-gray-300 text-gray-700 hover:bg-gray-50", base)
                }
            }}
            on:click=move |_| on_click.run(())
        >
            <div class="flex items-center space-x-2">
                <Show when=move || is_active.get()>
                    <span class="w-2 h-2 bg-blue-500 rounded-full"></span>
                </Show>
                <span>{label}</span>
            </div>
        </button>
    }
}
