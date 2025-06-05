use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-4xl font-bold text-center mb-8">"WIXOSS Card Database"</h1>
            <div class="flex justify-center">
                <A href="/card" attr:class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded-lg transition-colors">
                    "Browse Cards"
                </A>
            </div>
        </div>
    }
}