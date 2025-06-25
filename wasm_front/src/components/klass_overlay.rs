use crate::types::{build_klass_matrix, KlassFilter};
use leptos::prelude::*;

#[component]
pub fn KlassOverlay(
    klass_filter: RwSignal<KlassFilter>,
    is_open: ReadSignal<bool>,
    on_close: WriteSignal<bool>,
) -> impl IntoView {
    // クロージャーを作成してライフタイムの問題を回避
    let close_overlay = move || on_close.set(false);
    // Klassマトリックスデータを構築
    let klass_matrix = build_klass_matrix();

    view! {
        <Show when=move || is_open.get()>
            <div
                class="fixed inset-0 z-50 flex items-center justify-center"
                style="background-color: rgba(0, 0, 0, 0.75);"
                on:click=move |_| {
                    close_overlay();
                }
            >
            <div
                class="bg-white rounded-lg max-w-6xl w-full mx-4 max-h-[80vh] overflow-auto"
                on:click=move |e| e.stop_propagation()
            >
                <div class="p-6">
                    // ヘッダー
                    <div class="flex justify-between items-center mb-6">
                        <div>
                            <h2 class="text-xl font-bold text-gray-900">
                                "クラス（種族）選択"
                            </h2>
                            <p class="text-sm text-gray-600 mt-1">
                                "シグニ、クラフトシグニ、レゾナのクラスを選択できます"
                            </p>
                        </div>
                        <div class="flex items-center gap-3">
                            <button
                                class="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded-md border"
                                on:click=move |_| {
                                    klass_filter.update(|f| f.clear_all());
                                }
                            >
                                "すべてクリア"
                            </button>
                            <button
                                class="text-gray-400 hover:text-gray-600 text-2xl leading-none"
                                on:click=move |_| {
                                    close_overlay();
                                }
                            >
                                "×"
                            </button>
                        </div>
                    </div>

                    // Klassマトリックス
                    <div class="space-y-6">
                        {klass_matrix.clone().into_iter().map(|(system, types)| {
                            view! {
                                <div class="border rounded-lg p-4">
                                    <h3 class="font-semibold text-lg mb-3 text-gray-700">
                                        {system.clone()}
                                    </h3>

                                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-2">
                                        {types.into_iter().map(|klass_info| {
                                            let klass_id = klass_info.id;
                                            let display_name = klass_info.display_name();

                                            view! {
                                                <button
                                                    class=move || {
                                                        let is_selected = klass_filter.get().is_klass_selected(klass_id);
                                                        if is_selected {
                                                            "px-3 py-2 text-sm rounded-md border border-blue-500 bg-blue-50 text-blue-700 hover:bg-blue-100"
                                                        } else {
                                                            "px-3 py-2 text-sm rounded-md border border-gray-300 bg-white text-gray-700 hover:bg-gray-50"
                                                        }
                                                    }
                                                    on:click=move |_| {
                                                        klass_filter.update(|f| f.toggle_klass(klass_id));
                                                    }
                                                >
                                                    {display_name}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // フッター
                    <div class="flex justify-end mt-6">
                        <button
                            class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
                            on:click=move |_| {
                                close_overlay();
                            }
                        >
                            "適用"
                        </button>
                    </div>
                </div>
            </div>
            </div>
        </Show>
    }
}
