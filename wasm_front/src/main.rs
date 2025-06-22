use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod components;
mod pages;
mod types;

use components::SvgDefinition;
use pages::{CardPage, CardDetailPage, HomePage};

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <SpaRedirectHandler />
            <SvgDefinition />
            <div class="min-h-screen bg-gray-100">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/card") view=CardPage/>
                    <Route path=path!("/card/:param") view=CardDetailPage/>
                </Routes>
            </div>
        </Router>
    }
}

#[component]
fn SpaRedirectHandler() -> impl IntoView {
    use leptos_router::hooks::use_navigate;
    
    // Handle GitHub Pages 404 fallback using sessionStorage
    let navigate = use_navigate();
    
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(storage) = window.session_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(redirect_path)) = storage.get_item("spa_redirect_path") {
                        // Clear the stored path
                        let _ = storage.remove_item("spa_redirect_path");
                        
                        // Navigate to the stored path
                        navigate(&redirect_path, Default::default());
                    }
                }
            }
        }
    });

    // This component doesn't render anything
    view! { <span style="display: none;"></span> }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
