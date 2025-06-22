use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn ScrollToTopButton() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);
    
    // スクロール位置の監視とボタンの表示制御
    Effect::new(move |_| {
        let window = web_sys::window().expect("should have window");
        let window_clone = window.clone();
        
        let closure = Closure::wrap(Box::new(move || {
            let window_scroll_y = window_clone.scroll_y().unwrap_or(0.0);
            
            // 200pxスクロールしたら表示
            let should_show = window_scroll_y > 200.0;
            set_is_visible.set(should_show);
        }) as Box<dyn Fn()>);
        
        window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .expect("should add scroll event listener");
        
        // クリーンアップのためにclosureを忘れる
        closure.forget();
    });
    
    let scroll_to_top = move |_| {
        let window = web_sys::window().expect("should have window");
        window.scroll_to_with_x_and_y(0.0, 0.0);
    };
    
    view! {
        <Show when=move || is_visible.get()>
            <button
                class="fixed bottom-6 right-6 z-40 bg-blue-500 hover:bg-blue-600 text-white p-3 rounded-full shadow-lg transition-all duration-300 hover:scale-110"
                on:click=scroll_to_top
                title="一番上に戻る"
                aria-label="一番上に戻る"
            >
                <svg 
                    class="w-6 h-6" 
                    fill="none" 
                    stroke="currentColor" 
                    viewBox="0 0 24 24"
                >
                    <path 
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        stroke-width="2" 
                        d="M5 15l7-7 7 7"
                    />
                </svg>
            </button>
        </Show>
    }
}