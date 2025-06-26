use crate::components::svg_definition::{IconNext, IconPrevious};
use leptos::prelude::*;

// Simple browser detection
fn is_firefox() -> bool {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = "eval")]
        fn js_eval(code: &str) -> JsValue;
    }

    let user_agent = js_eval("navigator.userAgent");
    if let Some(ua_str) = user_agent.as_string() {
        return ua_str.to_lowercase().contains("firefox");
    }
    false
}

// Browser-aware scroll function to top pagination position
fn scroll_to_pagination() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            use wasm_bindgen::JsCast;

            // Try to find the top pagination element
            let top_pagination_element = document.query_selector("[data-pagination-top]");

            #[allow(unused)]
            let target_y = if let Ok(Some(element)) = top_pagination_element {
                // Use JavaScript to get element position
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_name = "eval")]
                    fn js_eval(code: &str) -> JsValue;
                }

                let js_code = r#"
                    (function() {
                        var element = document.querySelector('[data-pagination-top]');
                        if (element) {
                            var rect = element.getBoundingClientRect();
                            return rect.top + window.pageYOffset - 20;
                        }
                        return 100;
                    })();
                "#;

                let result = js_eval(js_code);
                result.as_f64().unwrap_or(100.0)
            } else {
                // Fallback: scroll to a reasonable position from top (not all the way to top)
                100.0
            };

            if is_firefox() {
                // Firefox: Force immediate scroll using multiple methods
                use wasm_bindgen::prelude::*;

                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_name = "eval")]
                    fn js_eval(code: &str) -> JsValue;
                }

                // Method 1: Direct JavaScript execution for immediate scroll
                let js_code = format!(
                    r#"
                    (function() {{
                        var html = document.documentElement;
                        var body = document.body;
                        var targetY = {};
                        
                        // Add Firefox instant scroll class
                        html.classList.add('firefox-instant-scroll');
                        
                        // Force immediate scroll without animation - multiple methods
                        html.style.scrollBehavior = 'auto';
                        body.style.scrollBehavior = 'auto';
                        
                        // Scroll using all available methods
                        window.scrollTo({{top: targetY, left: 0, behavior: 'auto'}});
                        window.scrollTo(0, targetY);
                        html.scrollTop = targetY;
                        body.scrollTop = targetY;
                        
                        // Force layout recalculation
                        html.offsetHeight;
                        body.offsetHeight;
                        
                        // Verify and retry if needed
                        setTimeout(function() {{
                            if (Math.abs(window.pageYOffset - targetY) > 5) {{
                                window.scrollTo(0, targetY);
                                html.scrollTop = targetY;
                                body.scrollTop = targetY;
                            }}
                            
                            // Restore smooth scrolling for other interactions
                            html.classList.remove('firefox-instant-scroll');
                            html.style.scrollBehavior = 'smooth';
                            body.style.scrollBehavior = 'smooth';
                        }}, 150);
                    }})();
                "#,
                    target_y
                );

                js_eval(&js_code);

                // Method 2: Additional Rust-side scroll as backup
                window.scroll_to_with_x_and_y(0.0, target_y);
            } else {
                // Chrome/Safari: Smooth animated scroll
                window.scroll_to_with_x_and_y(0.0, target_y);

                // Additional delayed scroll to ensure it works
                let timeout_closure = wasm_bindgen::closure::Closure::once(Box::new(move || {
                    if let Some(window) = web_sys::window() {
                        window.scroll_to_with_x_and_y(0.0, target_y);
                    }
                })
                    as Box<dyn FnOnce()>);

                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    timeout_closure.as_ref().unchecked_ref(),
                    100,
                );
                timeout_closure.forget();
            }
        }
    }
}

// Browser-aware scroll function to absolute top
fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        use wasm_bindgen::JsCast;

        if is_firefox() {
            // Firefox: Force immediate scroll using multiple methods
            use wasm_bindgen::prelude::*;

            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_name = "eval")]
                fn js_eval(code: &str) -> JsValue;
            }

            // Method 1: Direct JavaScript execution for immediate scroll
            js_eval(
                r#"
                (function() {
                    var html = document.documentElement;
                    var body = document.body;
                    
                    // Add Firefox instant scroll class
                    html.classList.add('firefox-instant-scroll');
                    
                    // Force immediate scroll without animation - multiple methods
                    html.style.scrollBehavior = 'auto';
                    body.style.scrollBehavior = 'auto';
                    
                    // Scroll using all available methods
                    window.scrollTo({top: 0, left: 0, behavior: 'auto'});
                    window.scrollTo(0, 0);
                    html.scrollTop = 0;
                    body.scrollTop = 0;
                    
                    // Force layout recalculation
                    html.offsetHeight;
                    body.offsetHeight;
                    
                    // Verify and retry if needed
                    setTimeout(function() {
                        if (window.pageYOffset > 0 || html.scrollTop > 0 || body.scrollTop > 0) {
                            window.scrollTo(0, 0);
                            html.scrollTop = 0;
                            body.scrollTop = 0;
                        }
                        
                        // Restore smooth scrolling for other interactions
                        html.classList.remove('firefox-instant-scroll');
                        html.style.scrollBehavior = 'smooth';
                        body.style.scrollBehavior = 'smooth';
                    }, 150);
                })();
            "#,
            );

            // Method 2: Additional Rust-side scroll as backup
            window.scroll_to_with_x_and_y(0.0, 0.0);
        } else {
            // Chrome/Safari: Smooth animated scroll
            window.scroll_to_with_x_and_y(0.0, 0.0);

            // Additional delayed scroll to ensure it works
            let timeout_closure = wasm_bindgen::closure::Closure::once(Box::new(move || {
                if let Some(window) = web_sys::window() {
                    window.scroll_to_with_x_and_y(0.0, 0.0);
                }
            })
                as Box<dyn FnOnce()>);

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_closure.as_ref().unchecked_ref(),
                100,
            );
            timeout_closure.forget();
        }
    }
}

#[component]
pub fn Pagination(
    current_page: ReadSignal<usize>,
    total_pages: Memo<usize>,
    set_current_page: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <div class="flex justify-center gap-2">
            <button
                class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300 disabled:cursor-not-allowed hover:bg-blue-600 active:bg-blue-700 active:scale-95 transition-all duration-150 ease-in-out shadow-md hover:shadow-lg"
                prop:disabled=move || current_page.get() == 0
                on:click=move |_| {
                    set_current_page.update(|p| {
                        if *p > 0 {
                            *p -= 1;
                        }
                    });
                    scroll_to_pagination();
                }
            >
                <IconPrevious />
            </button>

            <button
                class="px-4 py-2 bg-gray-100 rounded border shadow-sm hover:bg-gray-200 active:bg-gray-300 transition-colors duration-150 cursor-pointer"
                on:click=move |_| {
                    scroll_to_top();
                }
                title="Click to scroll to top"
            >
                {move || {
                    let current = current_page.get();
                    let total = total_pages.get();
                    format!("Page {} of {}", current + 1, total.max(1))
                }}
            </button>

            <button
                class="px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-300 disabled:cursor-not-allowed hover:bg-blue-600 active:bg-blue-700 active:scale-95 transition-all duration-150 ease-in-out shadow-md hover:shadow-lg"
                prop:disabled=move || {
                    let current = current_page.get();
                    let total = total_pages.get();
                    current + 1 >= total.max(1)
                }
                on:click=move |_| {
                    let total = total_pages.get();
                    set_current_page.update(|p| {
                        if *p + 1 < total {
                            *p += 1;
                        }
                    });
                    scroll_to_pagination();
                }
            >
                <IconNext />
            </button>
        </div>
    }
}
