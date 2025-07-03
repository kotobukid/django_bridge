use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct FilterContext {
    pub selected_features: RwSignal<HashMap<String, bool>>,
    pub selected_burst_features: RwSignal<HashMap<String, bool>>,
    pub toggle_feature: Callback<String>,
    pub toggle_burst_feature: Callback<String>,
}

impl FilterContext {
    pub fn new(
        selected_features: RwSignal<HashMap<String, bool>>,
        selected_burst_features: RwSignal<HashMap<String, bool>>,
        set_selected_feature_names: WriteSignal<Vec<String>>,
        set_selected_burst_feature_names: WriteSignal<Vec<String>>,
    ) -> Self {
        let toggle_feature = Callback::new(move |feature_name: String| {
            selected_features.update(|features| {
                if features.contains_key(&feature_name) {
                    features.remove(&feature_name);
                } else {
                    features.insert(feature_name, true);
                }
            });
            // selected_feature_namesも更新
            let updated_names: Vec<String> = selected_features.read().keys().cloned().collect();
            set_selected_feature_names.set(updated_names);
        });

        let toggle_burst_feature = Callback::new(move |feature_name: String| {
            selected_burst_features.update(|features| {
                if features.contains_key(&feature_name) {
                    features.remove(&feature_name);
                } else {
                    features.insert(feature_name, true);
                }
            });
            // selected_burst_feature_namesも更新
            let updated_names: Vec<String> =
                selected_burst_features.read().keys().cloned().collect();
            set_selected_burst_feature_names.set(updated_names);
        });

        Self {
            selected_features,
            selected_burst_features,
            toggle_feature,
            toggle_burst_feature,
        }
    }
}
