use leptos::prelude::*;

#[component]
pub fn SvgToggleSwitch(
    /// 現在の状態（true = ON/右側、false = OFF/左側）
    is_on: impl Fn() -> bool + 'static + Send + Sync + Clone,
    /// 状態変更時のコールバック
    on_toggle: impl Fn(bool) + 'static,
    /// ラベルテキスト
    label: String,
    /// 無効状態かどうか（オプション）
    #[prop(optional)]
    disabled: bool,
) -> impl IntoView {
    let is_on_1 = is_on.clone();
    let is_on_2 = is_on.clone();
    let is_on_3 = is_on.clone();
    let is_on_4 = is_on.clone();

    let track_fill = move || {
        if disabled {
            "#d1d5db" // gray-300
        } else {
            "#e5e7eb" // gray-200 - 常に薄いグレー（ON/OFF関係なく）
        }
    };

    let knob_x = move || if is_on_2() { 24.0 } else { 4.0 };

    view! {
        <div
            class=move || format!(
                "flex items-center space-x-3 cursor-pointer hover:bg-white rounded p-2 transition-colors {}",
                if disabled { "opacity-50 cursor-not-allowed" } else { "" }
            )
            on:click=move |_| {
                if !disabled {
                    on_toggle(!is_on_3())
                }
            }
        >
            // SVGトグルスイッチ部分
            <div class=move || format!(
                "relative {}",
                if disabled { "cursor-not-allowed" } else { "cursor-pointer" }
            )>
                <svg
                    width="50"
                    height="28"
                    viewBox="0 0 50 28"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    // 背景トラック（横長楕円）
                    <rect
                        x="2"
                        y="6"
                        width="46"
                        height="16"
                        rx="8"
                        ry="8"
                        fill=track_fill()
                        class="transition-colors duration-200 ease-in-out"
                    />

                    // スライダーグループ
                    <g transform=move || format!("translate({}, 0)", knob_x())
                       class="transition-transform duration-300 ease-in-out">
                        // 白い外側の円
                        <circle
                            cx="14"
                            cy="14"
                            r="10"
                            fill="white"
                            stroke="#e5e7eb"
                            stroke-width="1"
                            class="drop-shadow-md"
                        />
                        // 内側の青い円（ON状態のみ表示）
                        {move || {
                            if is_on_4() {
                                view! {
                                    <circle
                                        cx="14"
                                        cy="14"
                                        r="12"
                                        fill="#2563eb"
                                        class="transition-opacity duration-300 ease-in-out"
                                    />
                                }.into_any()
                            } else {
                                view! { <g></g> }.into_any()
                            }
                        }}
                    </g>
                </svg>
            </div>

            // ラベルテキスト
            <span class=move || format!(
                "text-sm text-gray-700 select-none {}",
                if disabled { "cursor-not-allowed" } else { "cursor-pointer" }
            )>
                {label}
            </span>
        </div>
    }
}
