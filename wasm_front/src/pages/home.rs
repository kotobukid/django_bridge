use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex flex-col items-center justify-center bg-gray-100">
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-5xl font-bold text-center mb-12 text-gray-800">
                    "WIXOSS Card Database"
                </h1>

                <div class="flex flex-col items-center space-y-4">
                    <A
                        href="/card"
                        attr:class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 px-12 rounded-lg transition-all transform hover:scale-105 text-xl shadow-md"
                    >
                        "カードを検索"
                    </A>

                    <A
                        href="/credits"
                        attr:class="text-gray-600 hover:text-gray-800 underline transition-colors text-sm"
                    >
                        "クレジット"
                    </A>
                </div>
            </div>
        </div>
    }
}
