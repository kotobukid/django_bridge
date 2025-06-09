use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod components;
mod pages;
mod types;

use components::SvgDefinition;
use pages::{CardPage, HomePage};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <SvgDefinition />
            <div class="min-h-screen bg-gray-100">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/card") view=CardPage/>
                </Routes>
            </div>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
