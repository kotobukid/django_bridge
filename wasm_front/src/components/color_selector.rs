use leptos::prelude::*;
use crate::types::ColorFilter;

#[component]
pub fn ColorSelector(
    color_filter: ReadSignal<ColorFilter>,
    set_color_filter: WriteSignal<ColorFilter>,
) -> impl IntoView {
    let toggle_color = move |color: &'static str| {
        set_color_filter.update(|filter| {
            match color {
                "white" => filter.white = !filter.white,
                "blue" => filter.blue = !filter.blue,
                "black" => filter.black = !filter.black,
                "red" => filter.red = !filter.red,
                "green" => filter.green = !filter.green,
                "colorless" => filter.colorless = !filter.colorless,
                _ => {}
            }
        });
    };
    
    let clear_all = move |_| {
        set_color_filter.set(ColorFilter::new());
    };
    
    let color_button = move |color: &'static str, label: &'static str, bg_class: &'static str| {
        let is_active = Memo::new(move |_| {
            let filter = color_filter.get();
            match color {
                "white" => filter.white,
                "blue" => filter.blue,
                "black" => filter.black,
                "red" => filter.red,
                "green" => filter.green,
                "colorless" => filter.colorless,
                _ => false,
            }
        });
        
        view! {
            <button
                class=move || {
                    let base = format!("color-selector-button {}", bg_class);
                    if is_active.get() {
                        format!("{} color-selector-active", base)
                    } else {
                        base
                    }
                }
                on:click=move |_| toggle_color(color)
            >
                {label}
            </button>
        }
    };

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <h2 class="text-lg font-semibold mb-4">"Color Filter"</h2>
            <div class="flex flex-wrap gap-2">
                {color_button("white", "White", "bg-white text-gray-800 border border-gray-300")}
                {color_button("blue", "Blue", "bg-blue-500 text-white")}
                {color_button("black", "Black", "bg-gray-900 text-white")}
                {color_button("red", "Red", "bg-red-500 text-white")}
                {color_button("green", "Green", "bg-green-500 text-white")}
                {color_button("colorless", "Colorless", "bg-gray-500 text-white")}
                
                <Show when=move || color_filter.get().has_any()>
                    <button
                        class="color-selector-button bg-gray-300 text-gray-700"
                        on:click=clear_all
                    >
                        "Clear All"
                    </button>
                </Show>
            </div>
        </div>
    }
}