use leptos::prelude::*;

#[component]
pub fn ToggleSwitch(
    /// 現在の状態（true = ON/右側、false = OFF/左側）
    is_on: bool,
    /// 状態変更時のコールバック
    on_toggle: impl Fn(bool) + 'static,
    /// ラベルテキスト
    label: String,
    /// 無効状態かどうか（オプション）
    #[prop(optional)]
    disabled: bool,
) -> impl IntoView {
    view! {
        <div
            class=move || format!(
                "flex items-center space-x-3 cursor-pointer hover:bg-white rounded p-2 transition-colors {}",
                if disabled { "opacity-50 cursor-not-allowed" } else { "" }
            )
            on:click=move |_| {
                if !disabled {
                    on_toggle(!is_on)
                }
            }
        >
            // トグルスイッチ部分
            <div class=move || format!(
                "relative {}",
                if disabled { "cursor-not-allowed" } else { "cursor-pointer" }
            )>
                // 背景トラック（横長楕円）
                <div class=move || format!(
                    "w-11 h-6 rounded-full transition-colors duration-200 ease-in-out {}",
                    if is_on { "bg-blue-600 hover:bg-blue-700" } else { "bg-gray-200 hover:bg-gray-300" }
                )>
                    // スライダー（白い円）
                    <div class=move || format!(
                        "absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow-md transform transition-all duration-300 ease-in-out {}",
                        if is_on { "translate-x-5" } else { "translate-x-0" }
                    )></div>
                </div>
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
