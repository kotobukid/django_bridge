use crate::types::LevelFilter;
use leptos::prelude::*;

#[component]
fn LevelButton(
    level: &'static str,
    level_filter: ReadSignal<LevelFilter>,
    on_click: impl Fn(&'static str) + 'static,
    bg_class: &'static str,
) -> impl IntoView {
    let is_active = Memo::new(move |_| level_filter.get().is_selected(level));

    view! {
        <button
            class=move || {
                let base = format!("color-selector-button {bg_class}");
                if is_active.get() {
                    format!("{base} color-selector-active")
                } else {
                    base
                }
            }
            on:click=move |_| on_click(level)
        >
            "Lv"{level}
        </button>
    }
}

#[component]
pub fn LevelSelector(
    level_filter: ReadSignal<LevelFilter>,
    set_level_filter: WriteSignal<LevelFilter>,
) -> impl IntoView {
    let toggle_level = move |level: &'static str| {
        set_level_filter.update(|filter| {
            filter.toggle_level(level.to_string());
        });
    };

    let clear_all = move |_| {
        set_level_filter.update(|filter| {
            filter.clear_all();
        });
    };

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <div class="flex items-center gap-2">
                <div class="flex flex-wrap gap-1">
                    // "-" ボタン（クリアボタンとして機能）
                    <button
                        class="color-selector-button bg-gray-300 text-gray-700"
                        on:click=clear_all
                    >
                        "全Lv"
                    </button>

                    // レベル 0 から 4 のトグルボタン
                    <LevelButton level="0" level_filter=level_filter on_click=toggle_level bg_class="bg-gray-200" />
                    <LevelButton level="1" level_filter=level_filter on_click=toggle_level bg_class="bg-blue-100" />
                    <LevelButton level="2" level_filter=level_filter on_click=toggle_level bg_class="bg-blue-200" />
                    <LevelButton level="3" level_filter=level_filter on_click=toggle_level bg_class="bg-blue-300" />
                    <LevelButton level="4" level_filter=level_filter on_click=toggle_level bg_class="bg-blue-400" />

                    <Show when=move || level_filter.get().has_any()>
                        <button
                            class="color-selector-button bg-gray-300 text-gray-700"
                            on:click=clear_all
                        >
                            "Clear"
                        </button>
                    </Show>
                </div>
            </div>
        </div>
    }
}
