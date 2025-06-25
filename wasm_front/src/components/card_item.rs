use crate::components::svg_definition::{
    ColorIconsWithNum, IconBlack, IconBlue, IconColorless, IconGreen, IconRed, IconWhite,
};
use crate::components::{SkillTextRenderer, BurstTextRenderer};
use datapack::CardExport;
use leptos::prelude::*;


fn get_colors_from_bits(bits: i32) -> Vec<i32> {
    let mut colors = Vec::new();
    if bits & (1 << 1) != 0 {
        colors.push(1);
    } // White
    if bits & (1 << 2) != 0 {
        colors.push(2);
    } // Blue
    if bits & (1 << 3) != 0 {
        colors.push(3);
    } // Red
    if bits & (1 << 4) != 0 {
        colors.push(4);
    } // Black
    if bits & (1 << 5) != 0 {
        colors.push(5);
    } // Green
    if bits & (1 << 6) != 0 {
        colors.push(6);
    } // Colorless
    colors
}

#[component]
pub fn Icons(
    #[prop(into)] colors: Vec<i32>
) -> impl IntoView {
    // Calculate width based on number of icons with 14px spacing (18px overlap from 32px)
    let width = if colors.is_empty() {
        0
    } else {
        32 + (colors.len() - 1) * 14
    };
    
    view! {
        <div class="color-icons" style=format!("width: {}px; flex-shrink: 0;", width)>
            {colors.into_iter().map(|color| {
                match color {
                    1 => view! { <IconWhite /> }.into_any(),
                    2 => view! { <IconBlue /> }.into_any(),
                    3 => view! { <IconRed /> }.into_any(),
                    4 => view! { <IconBlack /> }.into_any(),
                    5 => view! { <IconGreen /> }.into_any(),
                    6 => view! { <IconColorless /> }.into_any(),
                    _ => view! { <span></span> }.into_any(),
                }
            }).collect_view()}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Compact,
    Detailed,
}

impl ViewMode {
    pub fn from_string(s: &str) -> Self {
        match s {
            "detailed" => ViewMode::Detailed,
            _ => ViewMode::Compact, // Default to Compact for unknown values
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            ViewMode::Compact => "compact",
            ViewMode::Detailed => "detailed",
        }
    }
}

// localStorage utility functions
impl ViewMode {
    pub fn save_to_storage(&self) {
        use wasm_bindgen::prelude::*;
        
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = "eval")]
            fn js_eval(code: &str) -> JsValue;
        }
        
        let js_code = format!(
            r#"
            (function() {{
                try {{
                    localStorage.setItem('card_view_mode', '{}');
                }} catch (e) {{
                    console.warn('Failed to save view mode to localStorage:', e);
                }}
            }})();
            "#,
            self.to_string()
        );
        
        js_eval(&js_code);
    }

    pub fn load_from_storage() -> Self {
        use wasm_bindgen::prelude::*;
        
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = "eval")]
            fn js_eval(code: &str) -> JsValue;
        }
        
        let js_code = r#"
            (function() {
                try {
                    return localStorage.getItem('card_view_mode') || 'compact';
                } catch (e) {
                    console.warn('Failed to load view mode from localStorage:', e);
                    return 'compact';
                }
            })();
        "#;
        
        let result = js_eval(js_code);
        if let Some(value) = result.as_string() {
            ViewMode::from_string(&value)
        } else {
            ViewMode::Compact // Default to Compact
        }
    }
}

#[component]
pub fn CardItem(
    card: CardExport,
    #[prop(optional)] view_mode: Option<Signal<ViewMode>>,
) -> impl IntoView {
    // Use provided view_mode or default to Compact
    let view_mode = view_mode.unwrap_or_else(|| Signal::derive(|| ViewMode::Compact));
    
    // Calculate styles
    let color_style = datapack::bits_to_gradient_native(card.color() as i32);
    let card_url = card.build_url();
    
    // Get color theme for this card
    let primary_color_name = datapack::get_primary_color_name_from_bits(card.color());
    let color_theme = datapack::get_color_theme_native(&primary_color_name);
    
    // Create dynamic styles based on card color
    let (bg_color, border_color) = if let Some((_, accent, light)) = color_theme {
        (light.to_string(), accent.to_string())
    } else {
        ("#f0f0f0".to_string(), "#cccccc".to_string())
    };
    
    // Determine border class based on card type
    let border_class = match card.card_type() {
        5 | 6 => "card-border-black", // Signi (5) or Spell (6)
        _ => "card-border-white",
    };
    
    view! {
        <div class=format!("card-item {}", border_class) style=format!("{}; border-radius: 8px; padding: 16px; margin: 8px 0;", color_style)>
            {move || match view_mode.get() {
                ViewMode::Compact => view! {
                    <div class="flex justify-between items-start">
                        <div class="flex-1">
                            <div>
                                <h3 class="font-semibold text-lg" style="color: #374151;">
                                    <a href=card_url.clone() target="_blank" class="flex items-center gap-1">
                                        <Icons colors={get_colors_from_bits(card.color() as i32)} />
                                        {card.name()}
                                        {
                                            let cost = card.cost();
                                            if !cost.is_empty() {
                                                view! {
                                                    <div class="ml-2 flex items-center">
                                                        <ColorIconsWithNum code={cost} />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }
                                    </a>
                                </h3>
                                {
                                    let pronunciation = card.pronunciation();
                                    if !pronunciation.is_empty() {
                                        view! {
                                            <div class="text-sm mt-1 ml-4" style="color: #6b7280;">
                                                "<" {pronunciation} ">"
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }
                            </div>
                            <div class="flex items-center gap-2 text-sm mt-1" style="color: #374151; opacity: 0.8;">
                                <span>{card.code()}</span>
                                {
                                    // レベル表示
                                    let level = card.level();
                                    if !level.is_empty() {
                                        view! {
                                            <span class="bg-blue-100 text-blue-800 px-2 py-1 rounded text-xs font-medium">
                                                "Lv: " {level}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                {
                                    // パワー表示
                                    let power = card.power();
                                    if !power.is_empty() {
                                        view! {
                                            <span class="bg-red-100 text-red-800 px-2 py-1 rounded text-xs font-medium">
                                                "Pow: " {power}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                {
                                    // リミット表示
                                    let limit = card.limit();
                                    if !limit.is_empty() {
                                        view! {
                                            <span class="bg-purple-100 text-purple-800 px-2 py-1 rounded text-xs font-medium">
                                                "Lim: " {limit}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                                {
                                    // クラス表示
                                    let klass_names = datapack::extract_klass_names_from_bits(card.klass_bits());
                                    if !klass_names.is_empty() {
                                        view! {
                                            {klass_names.into_iter().map(|klass_name| {
                                                view! {
                                                    <span class="bg-green-100 text-green-800 px-2 py-1 rounded text-xs font-medium">
                                                        {klass_name}
                                                    </span>
                                                }
                                            }).collect_view()}
                                        }.into_any()
                                    } else {
                                        view! {}.into_any()
                                    }
                                }
                            </div>
                            {
                                let skill_text = card.skill_text();
                                if !skill_text.is_empty() {
                                    view! {
                                        <div class="mt-2 text-sm whitespace-pre-wrap" style="color: #374151;">
                                            <SkillTextRenderer skill_text={skill_text} />
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }
                            {
                                let burst_text = card.burst_text();
                                let has_burst = card.has_burst() > 0;
                                if has_burst && !burst_text.is_empty() {
                                    view! {
                                        <div class="mt-2 text-sm" style="background: #374151; color: white; padding: 8px; border-radius: 4px; display: flex; align-items: center; gap: 6px;">
                                            <svg viewBox="0 0 32 32" width="20" height="20" style="flex-shrink: 0;">
                                                <use href="#lb_white_wrapped" />
                                            </svg>
                                            <span style="line-height: 1.2;">
                                                <BurstTextRenderer burst_text={burst_text} />
                                            </span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }
                        </div>
                    </div>
                }.into_any(),
                ViewMode::Detailed => view! {
                    <div 
                        class="border-l-4 p-4 rounded"
                        style=format!("background-color: {}; border-left-color: {};", bg_color, border_color)
                    >
                        <div class="flex items-center gap-2 mb-3">
                            <span 
                                class="font-bold text-sm uppercase tracking-wide"
                                style="color: black;"
                            >
                                "🔍 Detailed Mode"
                            </span>
                        </div>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <div class="mb-2">
                                    <h3 class="font-semibold text-lg" style="color: #374151;">
                                        <a href=card_url.clone() target="_blank" class="flex items-center gap-1">
                                            <Icons colors={get_colors_from_bits(card.color() as i32)} />
                                            {card.name()}
                                            {
                                                let cost = card.cost();
                                                if !cost.is_empty() {
                                                    view! {
                                                        <div class="ml-2 flex items-center">
                                                            <ColorIconsWithNum code={cost} />
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }
                                            }
                                        </a>
                                    </h3>
                                    {
                                        let pronunciation = card.pronunciation();
                                        if !pronunciation.is_empty() {
                                            view! {
                                                <div class="text-sm mt-1 ml-4" style="color: #6b7280;">
                                                    "<" {pronunciation} ">"
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }
                                </div>
                                <div class="space-y-2">
                                    <div class="flex items-center gap-2 text-sm" style="color: #374151;">
                                        <span class="font-medium">Code:</span>
                                        <a 
                                            href=format!("/card/{}", card.code()) 
                                            target="_blank"
                                            class="bg-gray-100 hover:bg-blue-100 px-2 py-1 rounded text-blue-600 hover:text-blue-800 transition-colors cursor-pointer"
                                        >
                                            {card.code()}
                                        </a>
                                    </div>
                                    <div class="flex flex-wrap gap-2">
                                        {
                                            let level = card.level();
                                            if !level.is_empty() {
                                                view! {
                                                    <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm font-medium">
                                                        "Level: " {level}
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                        {
                                            let power = card.power();
                                            if !power.is_empty() {
                                                view! {
                                                    <span class="bg-red-100 text-red-800 px-3 py-1 rounded-full text-sm font-medium">
                                                        "Power: " {power}
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                        {
                                            let limit = card.limit();
                                            if !limit.is_empty() {
                                                view! {
                                                    <span class="bg-purple-100 text-purple-800 px-3 py-1 rounded-full text-sm font-medium">
                                                        "Limit: " {limit}
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                        {
                                            let klass_names = datapack::extract_klass_names_from_bits(card.klass_bits());
                                            if !klass_names.is_empty() {
                                                view! {
                                                    {klass_names.into_iter().map(|klass_name| {
                                                        view! {
                                                            <span class="bg-green-100 text-green-800 px-3 py-1 rounded-full text-sm font-medium">
                                                                {klass_name}
                                                            </span>
                                                        }
                                                    }).collect_view()}
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }
                                        }
                                    </div>
                                </div>
                            </div>
                            <div class="space-y-3">
                                {
                                    // CardFeatures display - combined with BurstFeatures
                                    let card_features = datapack::extract_card_features_from_bits(card.feature_bits1(), card.feature_bits2());
                                    let burst_features = datapack::extract_burst_features_from_bits(card.burst_bits());
                                    
                                    if !card_features.is_empty() || !burst_features.is_empty() {
                                        view! {
                                            <div class="bg-blue-50 p-3 rounded border border-blue-200">
                                                <h4 class="font-medium text-sm text-blue-800 mb-2">Card Features</h4>
                                                <div class="flex flex-wrap gap-1">
                                                    // CardFeatures (blue theme)
                                                    {card_features.into_iter().map(|(tag_label, feature_label)| {
                                                        view! {
                                                            <span class="inline-block bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs font-medium border border-blue-300">
                                                                {format!("{} > {}", tag_label, feature_label)}
                                                            </span>
                                                        }
                                                    }).collect_view()}
                                                    // BurstFeatures (black theme)
                                                    {burst_features.into_iter().map(|(tag_label, feature_label)| {
                                                        view! {
                                                            <span class="inline-block bg-gray-800 text-white px-2 py-1 rounded-full text-xs font-medium border border-gray-600">
                                                                {format!("{} > {}", tag_label, feature_label)}
                                                            </span>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }
                                {
                                    let skill_text = card.skill_text();
                                    if !skill_text.is_empty() {
                                        view! {
                                            <div class="bg-white p-3 rounded border">
                                                <h4 class="font-medium text-sm text-gray-600 mb-2">Skill Text</h4>
                                                <div class="text-sm whitespace-pre-wrap" style="color: #374151;">
                                                    <SkillTextRenderer skill_text={skill_text} />
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }
                                {
                                    let burst_text = card.burst_text();
                                    let has_burst = card.has_burst() > 0;
                                    if has_burst && !burst_text.is_empty() {
                                        view! {
                                            <div class="bg-gray-800 text-white p-3 rounded border">
                                                <div class="flex items-center gap-2 mb-2">
                                                    <svg viewBox="0 0 32 32" width="20" height="20" style="flex-shrink: 0;">
                                                        <use href="#lb_white_wrapped" />
                                                    </svg>
                                                    <h4 class="font-medium text-sm">Life Burst</h4>
                                                </div>
                                                <div class="text-sm" style="line-height: 1.4;">
                                                    <BurstTextRenderer burst_text={burst_text} />
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }
                            </div>
                        </div>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}