use leptos::prelude::*;

#[component]
pub fn ClearAllButton(
    #[prop(into)] is_active: Signal<bool>,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class={move || {
                let base = "px-4 py-2 rounded-lg border transition-colors duration-200 font-medium";
                if is_active.get() {
                    format!("{} bg-red-100 border-red-500 text-red-700 hover:bg-red-200", base)
                } else {
                    format!("{} bg-gray-100 border-gray-300 text-gray-500 cursor-not-allowed", base)
                }
            }}
            on:click=move |_| {
                if is_active.get() {
                    on_click.run(())
                }
            }
            disabled=move || !is_active.get()
        >
            "全クリア"
        </button>
    }
}
