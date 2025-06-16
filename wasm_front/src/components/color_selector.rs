use crate::components::svg_definition::{IconBlack, IconBlue, IconColorless, IconGreen, IconWhite};
use crate::components::IconRed;
use crate::types::ColorFilter;
use leptos::prelude::*;

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
        set_color_filter.update(|filter| match color {
            "white" => filter.white = !filter.white,
            "blue" => filter.blue = !filter.blue,
            "black" => filter.black = !filter.black,
            "red" => filter.red = !filter.red,
            "green" => filter.green = !filter.green,
            "colorless" => filter.colorless = !filter.colorless,
            _ => {}
        });
    };

    let clear_all = move |_| {
        set_color_filter.set(ColorFilter::new());
    };

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <div class="flex flex-wrap gap-1">
                <ColorButton color="white" color_filter=color_filter on_click=toggle_color bg_class="bg-amber-100 border border-gray-300">
                    <IconWhite />
                </ColorButton>
                <ColorButton color="red" color_filter=color_filter on_click=toggle_color bg_class="bg-rose-400">
                    <IconRed />
                </ColorButton>
                <ColorButton color="blue" color_filter=color_filter on_click=toggle_color bg_class="bg-sky-300">
                    <IconBlue />
                </ColorButton>
                <ColorButton color="green" color_filter=color_filter on_click=toggle_color bg_class="bg-emerald-400">
                    <IconGreen />
                </ColorButton>
                <ColorButton color="black" color_filter=color_filter on_click=toggle_color bg_class="bg-violet-400">
                    <IconBlack />
                </ColorButton>
                <ColorButton color="colorless" color_filter=color_filter on_click=toggle_color bg_class="bg-gray-300">
                    <IconColorless />
                </ColorButton>

                <Show when=move || color_filter.get().has_any()>
                    <button
                        class="color-selector-button bg-gray-300 text-gray-700"
                        on:click=clear_all
                    >
                        "Clear"
                    </button>
                </Show>
            </div>
        </div>
    }
}
