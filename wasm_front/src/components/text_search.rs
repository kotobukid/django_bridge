use leptos::prelude::*;
use leptos::html;

#[component]
pub fn TextSearch(
    search_text: ReadSignal<String>,
    set_search_text: WriteSignal<String>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();

    let clear_search = move |_| {
        set_search_text.set(String::new());
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        set_search_text.set(value);
    };

    let has_text = Memo::new(move |_| !search_text.get().is_empty());

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <div class="flex items-center gap-2">
                <div class="relative flex-1">
                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                        <svg 
                            class="h-4 w-4 text-gray-400" 
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24"
                        >
                            <path 
                                stroke-linecap="round" 
                                stroke-linejoin="round" 
                                stroke-width="2" 
                                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                            />
                        </svg>
                    </div>
                    <input
                        node_ref=input_ref
                        type="text"
                        class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                        placeholder="カード名、コード、読みで検索..."
                        value=move || search_text.get()
                        on:input=on_input
                    />
                    <Show when=move || has_text.get()>
                        <div class="absolute inset-y-0 right-0 pr-3 flex items-center">
                            <button
                                class="text-gray-400 hover:text-gray-600 focus:outline-none"
                                on:click=clear_search
                                title="検索をクリア"
                            >
                                <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path 
                                        stroke-linecap="round" 
                                        stroke-linejoin="round" 
                                        stroke-width="2" 
                                        d="M6 18L18 6M6 6l12 12"
                                    />
                                </svg>
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
            <Show when=move || has_text.get()>
                <div class="mt-2 text-xs text-gray-500">
                    "検索中: " {move || search_text.get()}
                </div>
            </Show>
        </div>
    }
}