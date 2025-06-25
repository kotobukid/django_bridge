use crate::components::svg_definition::{
    IconBlueArchive, IconDenonbu, IconDissona, IconNijisanji, IconPripara,
};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
fn FeatureButton(
    feature: &'static str,
    is_active: Signal<bool>,
    on_click: impl Fn(&'static str) + 'static,
    bg_class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            aria-label=feature
            title=feature
            class=move || {
                let base = format!("color-selector-button {}", bg_class);
                if is_active.get() {
                    format!("{} color-selector-active", base)
                } else {
                    base
                }
            }
            on:click=move |_| on_click(feature)
        >
            {children()}
        </button>
    }
}

#[component]
pub fn FeatureShortcuts(
    selected_features: RwSignal<HashMap<String, bool>>,
    on_feature_change: WriteSignal<Vec<String>>,
) -> impl IntoView {
    // Define the special features we want shortcuts for
    // Using the actual Japanese feature names from the datapack
    const SHORTCUT_FEATURES: &[&str] = &[
        "プリパラ",
        "にじさんじ",
        "ディソナ",
        "電音部",
        "ブルーアーカイブ",
    ];

    let toggle_feature = move |feature_name: &'static str| {
        // Perform all updates in a single update call to minimize reactive updates
        selected_features.update(|features| {
            let current = features.get(feature_name).copied().unwrap_or(false);
            if current {
                // If clicking an active feature, just turn it off
                features.remove(feature_name);
            } else {
                // If activating a feature, turn off all other shortcuts first
                for &key in SHORTCUT_FEATURES {
                    if key != feature_name {
                        features.remove(key);
                    }
                }
                // Then turn on the selected feature
                features.insert(feature_name.to_string(), true);
            }
        });

        // Notify parent of current selected features (this happens after the update)
        let selected_names: Vec<String> = selected_features
            .read()
            .iter()
            .filter_map(|(name, &selected)| if selected { Some(name.clone()) } else { None })
            .collect();
        on_feature_change.set(selected_names);
    };

    let clear_all = move |_| {
        // Clear only the shortcut features
        selected_features.update(|features| {
            for &feature_key in SHORTCUT_FEATURES {
                features.remove(feature_key);
            }
        });

        // Notify parent
        let selected_names: Vec<String> = selected_features
            .read()
            .iter()
            .filter_map(|(name, &selected)| if selected { Some(name.clone()) } else { None })
            .collect();
        on_feature_change.set(selected_names);
    };

    // Check if any shortcut features are active
    let has_any_shortcuts = Signal::derive(move || {
        let features = selected_features.read();
        SHORTCUT_FEATURES
            .iter()
            .any(|&key| features.get(key).copied().unwrap_or(false))
    });

    view! {
        <div class="bg-white rounded-lg shadow p-4">
            <div class="flex flex-wrap gap-1">
                <FeatureButton
                    feature="プリパラ"
                    is_active=Signal::derive(move || selected_features.read().get("プリパラ").copied().unwrap_or(false))
                    on_click=toggle_feature
                    bg_class="bg-fuchsia-300 border border-gray-300"
                >
                    <IconPripara />
                </FeatureButton>

                <FeatureButton
                    feature="にじさんじ"
                    is_active=Signal::derive(move || selected_features.read().get("にじさんじ").copied().unwrap_or(false))
                    on_click=toggle_feature
                    bg_class="bg-blue-200 border border-gray-300"
                >
                    <IconNijisanji />
                </FeatureButton>

                <FeatureButton
                    feature="ディソナ"
                    is_active=Signal::derive(move || selected_features.read().get("ディソナ").copied().unwrap_or(false))
                    on_click=toggle_feature
                    bg_class="bg-fuchsia-900 border border-gray-300"
                >
                    <IconDissona />
                </FeatureButton>

                <FeatureButton
                    feature="電音部"
                    is_active=Signal::derive(move || selected_features.read().get("電音部").copied().unwrap_or(false))
                    on_click=toggle_feature
                    bg_class="bg-black border border-gray-300"
                >
                    <IconDenonbu />
                </FeatureButton>

                <FeatureButton
                    feature="ブルーアーカイブ"
                    is_active=Signal::derive(move || selected_features.read().get("ブルーアーカイブ").copied().unwrap_or(false))
                    on_click=toggle_feature
                    bg_class="bg-black border border-gray-300"
                >
                    <IconBlueArchive />
                </FeatureButton>

                <Show when=move || has_any_shortcuts.get()>
                    <button
                        class="color-selector-button bg-gray-300 text-gray-700"
                        on:click=clear_all
                    >
                        "Clear"
                    </button>
                </Show>
            </div>
        </div>
    }
}
