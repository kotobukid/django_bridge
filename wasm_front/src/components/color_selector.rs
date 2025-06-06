use leptos::prelude::*;
use crate::types::ColorFilter;
use crate::components::IconRed;
use crate::components::svg_definition::{IconBlack, IconBlue, IconColorless, IconGreen, IconWhite};

#[component]
fn ColorButton(
    color: &'static str,
    color_filter: ReadSignal<ColorFilter>,
    on_click: impl Fn(&'static str) + 'static,
    bg_class: &'static str,
    children: Children,
) -> impl IntoView {
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
            on:click=move |_| on_click(color)
        >
            {children()}
        </button>
    }
}

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

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <h2 class="text-lg font-semibold mb-4">"Color Filter"</h2>
            <div class="flex flex-wrap gap-2">
                <ColorButton color="white" color_filter=color_filter on_click=toggle_color bg_class="bg-white text-gray-800 border border-gray-300">
                    <IconWhite />
                </ColorButton>
                <ColorButton color="blue" color_filter=color_filter on_click=toggle_color bg_class="bg-blue-500 text-white">
                    <IconBlue />
                </ColorButton>
                <ColorButton color="black" color_filter=color_filter on_click=toggle_color bg_class="bg-gray-900 text-white">
                    <IconBlack />
                </ColorButton>
                <ColorButton color="red" color_filter=color_filter on_click=toggle_color bg_class="bg-red-500 text-white">
                    <IconRed />
                </ColorButton>
                <ColorButton color="green" color_filter=color_filter on_click=toggle_color bg_class="bg-green-500 text-white">
                    <IconGreen />
                </ColorButton>
                <ColorButton color="colorless" color_filter=color_filter on_click=toggle_color bg_class="bg-gray-500 text-white">
                    <IconColorless />
                </ColorButton>
                
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