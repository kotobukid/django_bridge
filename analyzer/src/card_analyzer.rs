use crate::raw_card_analyzer::{AnalysisError, RawCardAnalyzer, to_half};
use chrono::{DateTime, Utc};
use models::card::CreateCard;
use models::r#gen::django_models::RawCardDb;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use feature::{create_detect_patterns, create_burst_detect_patterns, CardFeature, BurstFeature};
use feature::feature::{HashSetToBits, BurstHashSetToBits};
use std::collections::HashSet;
use color::convert_cost;
use rayon::prelude::*;

/// CreateCard with detected Klass information
#[derive(Debug, Clone)]
pub struct CreateCardWithKlass {
    pub create_card: CreateCard,
    pub detected_klasses: Vec<(String, Option<String>, Option<String>)>,
}

/// RawCardDb with product_id included
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct RawCardWithProduct {
    pub id: i64,
    pub card_number: String,
    pub name: String,
    pub raw_html: String,
    pub skill_text: String,
    pub life_burst_text: String,
    pub source_url: String,
    pub scraped_at: DateTime<Utc>,
    pub last_analyzed_at: Option<DateTime<Utc>>,
    pub is_analyzed: bool,
    pub analysis_error: String,
    pub product_id: Option<i64>,
}

impl RawCardWithProduct {
    /// Convert to RawCardDb for compatibility
    pub fn to_raw_card_db(&self) -> RawCardDb {
        RawCardDb {
            id: self.id,
            card_number: self.card_number.clone(),
            name: self.name.clone(),
            raw_html: self.raw_html.clone(),
            skill_text: self.skill_text.clone(),
            life_burst_text: self.life_burst_text.clone(),
            source_url: self.source_url.clone(),
            scraped_at: self.scraped_at,
            last_analyzed_at: self.last_analyzed_at,
            is_analyzed: self.is_analyzed,
            analysis_error: self.analysis_error.clone(),
        }
    }
}

/// シンプルなRawCardAnalyzer実装
/// 基本的な解析を行う（色情報も含む）
pub struct SimpleRawCardAnalyzer;

impl SimpleRawCardAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// HTMLからカード種別を簡易検出する
    /// データベースのwix_cardtypeテーブルのIDに対応
    fn detect_card_type_from_html(&self, html: &str) -> i32 {
        // カード種別セクションを探してそこから種別を抽出
        let card_type_text = if let Some(start) = html.find("<dt>カード種類</dt>") {
            let after_dt = &html[start..];
            if let Some(dd_start) = after_dt.find("<dd>") {
                if let Some(dd_end) = after_dt.find("</dd>") {
                    &after_dt[dd_start + 4..dd_end]
                } else {
                    ""
                }
            } else {
                ""
            }
        } else {
            // フォールバック: 旧形式のHTML構造を試す
            if let Some(start) = html.find("<dt>種類</dt>") {
                let after_dt = &html[start..];
                if let Some(dd_start) = after_dt.find("<dd>") {
                    if let Some(dd_end) = after_dt.find("</dd>") {
                        &after_dt[dd_start + 4..dd_end]
                    } else {
                        ""
                    }
                } else {
                    ""
                }
            } else {
                ""
            }
        };

        // println!("DEBUG: Extracted card type text: '{}'", card_type_text);

        // 抽出したテキストから種別を判定
        if card_type_text.contains("ルリグ") {
            if card_type_text.contains("アシスト") {
                3 // アシストルリグ (DB ID: 3)
            } else {
                1 // ルリグ (DB ID: 1)
            }
        } else if card_type_text.contains("アーツ") {
            if card_type_text.contains("クラフト") {
                9 // クラフトアーツ (DB ID: 9)
            } else {
                2 // アーツ (DB ID: 2)
            }
        } else if card_type_text.contains("キー") {
            8 // キー (DB ID: 8)
        } else if card_type_text.contains("シグニ") {
            if card_type_text.contains("クラフト") {
                10 // クラフトシグニ (DB ID: 10)
            } else {
                5 // シグニ (DB ID: 5)
            }
        } else if card_type_text.contains("スペル") {
            if card_type_text.contains("クラフト") {
                11 // クラフトスペル (DB ID: 11)
            } else {
                6 // スペル (DB ID: 6)
            }
        } else if card_type_text.contains("レゾナ") {
            if card_type_text.contains("クラフト") {
                14 // クラフトレゾナ (DB ID: 14)
            } else {
                7 // レゾナ (DB ID: 7)
            }
        } else if card_type_text.contains("ピース") {
            if card_type_text.contains("リレー") {
                12 // リレーピース (DB ID: 12)
            } else if card_type_text.contains("クラフト") {
                13 // クラフトピース (DB ID: 13)
            } else {
                4 // ピース (DB ID: 4)
            }
        } else if card_type_text.contains("トークン") {
            15 // トークン (DB ID: 15)
        } else if card_type_text.contains("コイン") {
            16 // コイン (DB ID: 16)
        } else {
            // カード種別セクションが見つからない場合は全体のHTMLから検索（フォールバック）
            // println!("DEBUG: Card type section not found, falling back to full HTML search");
            if html.contains("ルリグ") && !html.contains("ルリグトラッシュ") {
                if html.contains("アシスト") {
                    3 // アシストルリグ (DB ID: 3)
                } else {
                    1 // ルリグ (DB ID: 1)
                }
            } else if html.contains("アーツ") {
                if html.contains("クラフト") {
                    9 // クラフトアーツ (DB ID: 9)
                } else {
                    2 // アーツ (DB ID: 2)
                }
            } else {
                0 // Unknown
            }
        }
    }

    /// HTMLから色を検出する
    fn detect_color_from_html(&self, html: &str) -> i32 {
        let mut color = 0;

        // <dt>色</dt><dd>○○</dd> パターンで色を検出
        if let Some(color_start) = html.find("<dt>色</dt>") {
            let after_dt = &html[color_start + 11..]; // "<dt>色</dt>"の後
            if let Some(dd_start) = after_dt.find("<dd>") {
                if let Some(dd_end) = after_dt.find("</dd>") {
                    let color_text = &after_dt[dd_start + 4..dd_end];

                    // 各色をチェック（shared/color/lib.rsの定義に合わせる）
                    if color_text.contains("白") {
                        color |= 1 << 1; // White
                    }
                    if color_text.contains("青") {
                        color |= 1 << 2; // Blue
                    }
                    if color_text.contains("赤") {
                        color |= 1 << 3; // Red
                    }
                    if color_text.contains("黒") {
                        color |= 1 << 4; // Black
                    }
                    if color_text.contains("緑") {
                        color |= 1 << 5; // Green
                    }
                    if color_text.contains("無") {
                        color = 1 << 6; // Colorless
                    }
                }
            }
        }

        // 何も検出されない場合はデフォルトで不明
        if color == 0 {
            color = 1 << 7; // Unknown
        }

        color
    }

    /// HTMLから共通のdd要素を抽出するヘルパーメソッド
    fn extract_dd_elements(&self, html: &str) -> Vec<String> {
        use regex::Regex;
        
        // dt-dd ペアを正しく抽出するための正規表現（複数行対応）
        let dt_dd_regex = Regex::new(r"(?s)<dt>([^<]+)</dt>\s*<dd[^>]*>(.*?)</dd>").unwrap();
        let mut dd_elements = Vec::new();
        
        // まず最初のdt要素（カード種類または種類）を見つける
        let start_patterns = ["<dt>カード種類</dt>", "<dt>種類</dt>"];
        let mut search_start = 0;
        
        for pattern in &start_patterns {
            if let Some(index) = html.find(pattern) {
                search_start = index;
                break;
            }
        }
        
        // search_start以降でdt-ddペアを抽出
        let search_html = &html[search_start..];
        
        for caps in dt_dd_regex.captures_iter(search_html) {
            if let (Some(_dt_content), Some(dd_content)) = (caps.get(1), caps.get(2)) {
                let dd_text = dd_content.as_str().trim();
                dd_elements.push(dd_text.to_string());
            }
        }
        dd_elements
    }

    /// HTMLからレベル情報を検出する（dd[3]）
    pub fn detect_level_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.len() > 3 {
            let level_text = dd_elements[3].trim();
            if !level_text.is_empty() && level_text != "-" {
                Some(level_text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// HTMLからリミット情報を検出する（dd[6]、ルリグ、アシストルリグのみ）
    pub fn detect_limit_from_html(&self, html: &str) -> (Option<String>, HashSet<CardFeature>) {
        let dd_elements = self.extract_dd_elements(html);

        let mut feature_set = HashSet::new();

        if dd_elements.is_empty() {
            return (None, feature_set);
        }

        // ルリグカードかどうかをチェック
        let card_type = &dd_elements[0];
        let is_lrig = card_type.contains("ルリグ") || card_type.contains("アシストルリグ");


        if is_lrig && dd_elements.len() > 6 {
            let limit_text = dd_elements[6].trim();
            if !limit_text.is_empty() && limit_text != "-" {


                if card_type.contains("アシストルリグ") {
                    if limit_text != "0" {
                        feature_set.insert(CardFeature::EnhanceLimit);
                    }
                }
                
                (Some(limit_text.to_string()), feature_set)
            } else {
                (None, feature_set)
            }
        } else {
            (None, feature_set)
        }
    }

    /// HTMLからパワー情報を検出する（dd[7]、シグニ/クラフトのみ）
    pub fn detect_power_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.is_empty() {
            return None;
        }

        // シグニ、クラフト、レゾナカードかどうかをチェック
        let card_type = &dd_elements[0];
        let has_power = card_type.contains("シグニ") || card_type.contains("クラフト") || card_type.contains("レゾナ");

        if has_power && dd_elements.len() > 7 {
            let power_text = dd_elements[7].trim();
            if !power_text.is_empty() && power_text != "-" {
                Some(power_text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// HTMLから使用タイミング情報を検出する（dd[9]、アーツ/ピースのみ）
    pub fn detect_timing_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.is_empty() {
            return None;
        }

        // アーツまたはピースカードかどうかをチェック
        let card_type = &dd_elements[0];
        let is_arts_or_piece = card_type.contains("アーツ") || card_type.contains("ピース");

        if is_arts_or_piece && dd_elements.len() > 9 {
            let timing_text = dd_elements[9].trim();
            if !timing_text.is_empty() && timing_text != "-" {
                Some(timing_text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// HTMLからリミット消費情報を検出する（dd[8]、将来対応）
    pub fn detect_limit_ex_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.len() > 8 {
            let limit_ex_text = dd_elements[8].trim();
            if !limit_ex_text.is_empty() && limit_ex_text != "-" {
                Some(limit_ex_text.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// HTMLからストーリー情報を検出する（dd[11]のdissonaアイコンチェック）
    pub fn detect_story_from_html(&self, html: &str) -> (Option<String>, HashSet<CardFeature>) {
        let dd_elements = self.extract_dd_elements(html);

        let mut s: HashSet<CardFeature> = HashSet::new();

        if dd_elements.len() > 11 {
            let story_html = &dd_elements[11];

            // dd[11]内にdissonaアイコンがあるかチェック
            if story_html.contains("icon_txt_dissona.png") {
                s.insert(CardFeature::Dissona);
                (Some("dissona".to_string()), s)
            } else {
                (None, s)
            }
        } else {
            (None, s)
        }
    }

    /// HTMLからフォーマット情報を検出する（dd[10]）
    /// 包含関係: オールスター（空文字）> キーセレクション > ディーヴァセレクション
    /// 対応値: 7（オールスター）, 3（キーセレクション）, 1（ディーヴァセレクション）
    pub fn detect_format_from_html(&self, html: &str) -> i32 {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.len() > 10 {
            let format_html = &dd_elements[10];

            // フォーマットアイコンをチェック
            let has_key_icon = format_html.contains("icon_txt_format_key.png");
            let has_diva_icon = format_html.contains("icon_txt_format_diva.png");

            // 包含関係に基づいて値を決定
            if has_diva_icon {
                1 // ディーヴァセレクション（001）
            } else if has_key_icon {
                3 // キーセレクション（011）
            } else {
                7 // オールスター（111）- デフォルト（空文字の場合）
            }
        } else {
            7 // デフォルトはオールスター
        }
    }

    /// 公式サイトの表記ゆれ・誤字を修正する
    fn normalize_klass(&self, cat1: &str, cat2: Option<&str>, cat3: Option<&str>) -> (String, Option<String>, Option<String>) {
        let mut normalized_cat1 = cat1.to_string();
        let mut normalized_cat2 = cat2.map(|s| s.to_string());
        let mut normalized_cat3 = cat3.map(|s| s.to_string());
        
        // 公式誤字修正: 奏生：植物 → 奏羅：植物
        if cat1 == "奏生" && cat2 == Some("植物") {
            normalized_cat1 = "奏羅".to_string();
        }
        
        // 表記ゆれ修正: ウエポン → ウェポン
        if let Some(ref mut cat2_val) = normalized_cat2 {
            if cat2_val == "ウエポン" {
                *cat2_val = "ウェポン".to_string();
            }
        }
        
        // バーチャル/世怜音女学院 → バーチャルのみ
        if let Some(ref cat2_val) = normalized_cat2 {
            if cat2_val == "バーチャル" && cat3 == Some("世怜音女学院") {
                normalized_cat3 = None;
            }
        }
        
        (normalized_cat1, normalized_cat2, normalized_cat3)
    }

    /// HTMLからKlass情報を検出する（dd[1]のカードタイプから）
    pub fn detect_klass_from_html(&self, html: &str) -> Vec<(String, Option<String>, Option<String>)> {
        let dd_elements = self.extract_dd_elements(html);
        let mut klasses = Vec::new();

        // dd[1]にカードタイプがある（シグニの場合は種族情報を含む）
        if dd_elements.len() > 1 {
            let card_type_text = &dd_elements[1];
            
            // <br>タグで分割してそれぞれ処理
            let lines: Vec<&str> = card_type_text
                .split("<br>")
                .flat_map(|line| line.split("<br/>"))
                .flat_map(|line| line.split("<br />"))
                .collect();

            for line in lines {
                let clean_text = line.trim();
                if clean_text.is_empty() || clean_text == "-" {
                    continue;
                }

                // パターン1: "奏羅：宇宙" のような形式（半角コロン）
                if let Some(colon_pos) = clean_text.find(':') {
                    let cat1 = clean_text[..colon_pos].trim();
                    let rest = clean_text[colon_pos + 1..].trim();
                    
                    // パターン1a: "空獣／地獣" のような複数cat2を持つ場合
                    if let Some(slash_pos) = rest.find('/') {
                        let cat2 = rest[..slash_pos].trim();
                        let cat3 = rest[slash_pos + 1..].trim();
                        klasses.push((cat1.to_string(), Some(cat2.to_string()), Some(cat3.to_string())));
                    } else {
                        // パターン1b: "奏羅：宇宙" のような単一cat2の場合
                        klasses.push((cat1.to_string(), Some(rest.to_string()), None));
                    }
                }
                // パターン2: 全角コロン "奏羅：宇宙" のような形式
                else if let Some(colon_pos) = clean_text.find('：') {
                    let cat1 = clean_text[..colon_pos].trim();
                    let rest = clean_text[colon_pos + '：'.len_utf8()..].trim();
                    
                    // パターン2a: "空獣／地獣" のような複数cat2を持つ場合
                    if let Some(slash_pos) = rest.find('/') {
                        let cat2 = rest[..slash_pos].trim();
                        let cat3 = rest[slash_pos + 1..].trim();
                        klasses.push((cat1.to_string(), Some(cat2.to_string()), Some(cat3.to_string())));
                    } else {
                        // パターン2b: "奏羅：宇宙" のような単一cat2の場合
                        klasses.push((cat1.to_string(), Some(rest.to_string()), None));
                    }
                }
                // パターン3: 単純な種族名のみ（解放派、闘争派、防衛派、奏元、精元など）
                else {
                    klasses.push((clean_text.to_string(), None, None));
                }
            }
        }

        // 正規化処理を適用
        let normalized_klasses: Vec<(String, Option<String>, Option<String>)> = klasses
            .into_iter()
            .map(|(cat1, cat2, cat3)| {
                self.normalize_klass(&cat1, cat2.as_deref(), cat3.as_deref())
            })
            .collect();
        
        normalized_klasses
    }

    pub fn detect_story_from_name(name: &str) -> HashSet<CardFeature> {
        let mut s: HashSet<CardFeature> = HashSet::new();
        if name.starts_with("電音部") {
            s.insert(CardFeature::Denonbu);
            return s;
        }
        if name.starts_with("プリパラアイドル") { 
            s.insert(CardFeature::Pripara);
            return s;
        }
        if name.starts_with("コード2434") {
            s.insert(CardFeature::Nijisanji);
        }
        s
    }

    /// HTMLからコスト情報を検出する（ルリグはグロウコスト、それ以外はコスト）
    pub fn detect_cost_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        // dd要素が十分にない場合は早期リターン
        if dd_elements.is_empty() {
            return None;
        }

        // カードタイプを確認（dd[0]がカード種類）
        let card_type = &dd_elements[0];
        let is_lrig = card_type.contains("ルリグ");

        // ルリグの場合はグロウコスト（dd[4]）、それ以外はコスト（dd[5]）
        let cost_index = if is_lrig { 4 } else { 5 };

        if dd_elements.len() > cost_index {
            let cost_html = &dd_elements[cost_index];

            // HTMLタグを除去（flatten_breakと同等の処理）
            let cost_text = cost_html
                .replace('\n', "")
                .replace("<br>", "")
                .replace("<br/>", "")
                .replace("<br />", "");

            if !cost_text.trim().is_empty() && cost_text.trim() != "-" {
                // convert_cost関数を使って「《白》×１《青》×２」→「w1u2」形式に変換
                match convert_cost(&cost_text) {
                    Ok(converted) => Some(converted),
                    Err(_) => {
                        // 変換に失敗した場合は元のテキストを返す
                        println!("DEBUG: Failed to convert {} '{}' for {} card",
                                if is_lrig { "grow cost" } else { "cost" },
                                cost_text,
                                if is_lrig { "Lrig" } else { "non-Lrig" });
                        Some(cost_text)
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// スキルテキストから特徴を検出し、置換後のテキストも返す
    fn detect_features_and_replace_text(&self, skill_text: &str) -> (i64, i64, String) {
        let (replace_patterns, detect_patterns) = create_detect_patterns();
        let mut detected_features = HashSet::new();

        // 半角に変換
        let mut processed_skill_text = to_half(skill_text);

        // 置換パターンを適用して特徴を検出
        for pattern in &replace_patterns {
            // スキルテキストに対して置換を適用
            if pattern.pattern_r.is_match(&processed_skill_text) {
                processed_skill_text = pattern.pattern_r.replace_all(&processed_skill_text, pattern.replace_to).to_string();
                for feature in pattern.features_detected {
                    detected_features.insert(feature.clone());
                }
            }
        }

        // 検出パターンで特徴を検出（置換後のテキストに対して）- 並列処理
        let additional_features: HashSet<CardFeature> = detect_patterns
            .par_iter()
            .filter(|pattern| pattern.pattern_r.is_match(&processed_skill_text))
            .flat_map(|pattern| pattern.features_detected.par_iter().cloned())
            .collect();
        
        detected_features.extend(additional_features);

        // HashSetからビットに変換（feature crateの標準実装を使用）
        let (bits1, bits2) = detected_features.to_bits();

        (bits1, bits2, processed_skill_text)
    }

    /// ライフバーストテキストからバーストフィーチャーを検出（正規表現パターンシステム使用）
    fn detect_burst_features(&self, life_burst_text: &str) -> (i64, String) {
        let (replace_patterns, detect_patterns) = create_burst_detect_patterns();
        let mut detected_burst_features: HashSet<BurstFeature> = HashSet::new();

        // 半角に変換
        let mut processed_text = to_half(life_burst_text);

        // 置換パターンを適用してバーストフィーチャーを検出
        for pattern in &replace_patterns {
            if pattern.pattern_r.is_match(&processed_text) {
                processed_text = pattern.pattern_r.replace_all(&processed_text, pattern.replace_to).to_string();
                for feature in pattern.features_detected {
                    detected_burst_features.insert(feature.clone());
                }
            }
        }

        // 検出パターンでバーストフィーチャーを検出 - 並列処理
        let additional_features: HashSet<BurstFeature> = detect_patterns
            .par_iter()
            .filter(|pattern| pattern.pattern_r.is_match(&processed_text))
            .flat_map(|pattern| pattern.features_detected.par_iter().cloned())
            .collect();
        
        detected_burst_features.extend(additional_features);

        // HashSetからビットに変換
        (detected_burst_features.to_burst_bits(), processed_text)
    }


    pub async fn analyze_with_product_id(
        &self,
        raw_card: &RawCardDb,
        product_id: Option<i64>,
    ) -> Result<CreateCardWithKlass, AnalysisError> {
        // HTMLからカード種別を検出
        let card_type = self.detect_card_type_from_html(&raw_card.raw_html);
        // println!("DEBUG: Card {} - Detected card_type: {}", raw_card.card_number, card_type);

        // HTMLから色を検出
        let color = self.detect_color_from_html(&raw_card.raw_html);
        // println!("DEBUG: Card {} - Detected color: {}", raw_card.card_number, color);

        // HTMLからコストを検出
        let cost = self.detect_cost_from_html(&raw_card.raw_html);

        // HTMLから追加フィールドを検出
        let level_str = self.detect_level_from_html(&raw_card.raw_html);
        let (limit_str, detected_feature_set) = self.detect_limit_from_html(&raw_card.raw_html);
        let power = self.detect_power_from_html(&raw_card.raw_html);
        let timing_str = self.detect_timing_from_html(&raw_card.raw_html);
        let limit_ex_str = self.detect_limit_ex_from_html(&raw_card.raw_html);
        let (story, story_as_skill) = self.detect_story_from_html(&raw_card.raw_html);
        let detected_klasses = self.detect_klass_from_html(&raw_card.raw_html);
        let format = self.detect_format_from_html(&raw_card.raw_html);

        // 数値フィールドの型変換（String → i32）
        let level: Option<i32> = level_str.and_then(|s| s.parse().ok());
        let limit: Option<i32> = limit_str.and_then(|s| s.parse().ok());
        let limit_ex: Option<i32> = limit_ex_str.and_then(|s| s.parse().ok());

        // タイミングは数値ではなく文字列のままにしておく（"メインフェイズ"等）
        // 将来的に数値コード化が必要であれば別途マッピング処理を追加
        let timing: Option<i32> = timing_str.and_then(|s| {
            // タイミング文字列を数値にマッピング（仮の実装）
            match s.as_str() {
                "メインフェイズ" => Some(1),
                "アタックフェイズ" => Some(2),
                _ => None,
            }
        });

        // スキルテキストとライフバーストテキストから特徴を検出し、置換後のテキストを取得
        let (mut feature_bits1, mut feature_bits2, replaced_skill_text) =
            self.detect_features_and_replace_text(&raw_card.skill_text);

        // ライフバーストテキストからバーストフィーチャーを検出
        let (burst_bits, replaced_burst_text) = self.detect_burst_features(&raw_card.life_burst_text);
        
        // ライフバーストテキストからもCardFeatureを検出
        let (burst_feature_bits1, burst_feature_bits2, _) = self.detect_features_and_replace_text(&raw_card.life_burst_text);
        feature_bits1 = feature_bits1 | burst_feature_bits1;
        feature_bits2 = feature_bits2 | burst_feature_bits2;

        {
            // テキスト以外からのFeature検出
            let bits = detected_feature_set.to_bits();
            feature_bits1 = feature_bits1 | bits.0;
            feature_bits2 = feature_bits2 | bits.1;
        }

        {
            let bits = story_as_skill.to_bits();
            feature_bits1 = feature_bits1 | bits.0;
            feature_bits2 = feature_bits2 | bits.1;
        }

        let full_name = to_half(&raw_card.name);
        let mut name: String;
        let mut pronunciation: String;
        
        // <読み方>形式を検索して分離
        if let Some(start) = full_name.find('<') {
            if let Some(end) = full_name.find('>') {
                // 読み方を抽出（<>を除く）
                pronunciation = full_name[start + 1..end].to_string();
                // nameから<読み方>部分を削除（前後の空白も削除）
                name = full_name[..start].trim_end().to_string();
            } else {
                // 閉じ括弧がない場合はフォールバック
                name = full_name.clone();
                pronunciation = full_name.clone();
            }
        } else {
            // <読み方>形式ではない場合はフォールバック
            name = full_name.clone();
            pronunciation = full_name.clone();
        }
        
        // nameが256バイトを超える場合は切り詰める
        if name.len() > 256 {
            name = name.chars()
                .scan(0, |acc, c| {
                    let char_len = c.len_utf8();
                    if *acc + char_len <= 256 {
                        *acc += char_len;
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect();
        }
        
        {
            let skills = Self::detect_story_from_name(&name);
            let bits = skills.to_bits();
            feature_bits1 = feature_bits1 | bits.0;
            feature_bits2 = feature_bits2 | bits.1;
        }
        // pronunciation が128文字を超える場合は切り詰める（バイト長ベース）
        if pronunciation.len() > 128 {
            // eprintln!("DEBUG: Truncating pronunciation from {} to 128 bytes", pronunciation.len());
            // UTF-8文字の境界を考慮して切り詰める
            pronunciation = pronunciation.chars()
                .collect::<String>()
                .chars()
                .scan(0, |acc, c| {
                    let char_len = c.len_utf8();
                    if *acc + char_len <= 128 {
                        *acc += char_len;
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect();
            // eprintln!("DEBUG: After truncation: {} bytes", pronunciation.len());
        }
        
        let create_card = CreateCard {
            name,
            code: to_half(&raw_card.card_number),
            pronunciation,
            color,
            cost,
            level,
            limit,
            limit_ex,
            product: product_id.unwrap_or(0) as i32,
            card_type,
            power,
            has_burst: if raw_card.life_burst_text.is_empty() {
                2
            } else {
                1
            },
            skill_text: Some(replaced_skill_text),
            burst_text: Some(replaced_burst_text),
            format,
            story,
            rarity: None,
            timing,
            url: Some(raw_card.source_url.clone()),
            feature_bits1,
            feature_bits2,
            burst_bits,
            ex1: None,
        };

        Ok(CreateCardWithKlass {
            create_card,
            detected_klasses,
        })
    }
}

#[async_trait::async_trait]
impl RawCardAnalyzer for SimpleRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        let result = self.analyze_with_product_id(raw_card, None).await?;
        Ok(result.create_card)
    }
}

/// カードをデータベースに保存する機能
pub struct CardRepository {
    pool: Arc<Pool<Postgres>>,
}

impl CardRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self { pool }
    }
    
    /// 不正なKlass形式を検証
    fn is_invalid_klass(&self, cat1: &str, cat2: Option<&str>) -> bool {
        // 色のみのKlass（白、赤、青、緑、黒）は除外
        if cat2.is_none() && matches!(cat1, "白" | "赤" | "青" | "緑" | "黒") {
            return true;
        }
        
        // 1文字だけのcat1（解析ミス）を除外
        if cat2.is_none() && cat1.chars().count() == 1 {
            return true;
        }
        
        // 正しい形式のcat1をチェック
        let valid_cat1_prefixes = [
            "奏像", "奏武", "奏羅", "奏械", "奏生", "奏元",
            "精像", "精武", "精羅", "精械", "精生", "精元",
            "解放派", "闘争派", "防衛派"
        ];
        
        // cat2がない場合は、有効なcat1のリストに含まれている必要がある
        if cat2.is_none() {
            return !valid_cat1_prefixes.contains(&cat1);
        }
        
        // cat2がある場合は、cat1が適切なプレフィックスである必要がある
        let valid_cat1_with_cat2 = [
            "奏像", "奏武", "奏羅", "奏械", "奏生",
            "精像", "精武", "精羅", "精械", "精生"
        ];
        
        if !valid_cat1_with_cat2.contains(&cat1) {
            return true;
        }
        
        false
    }

    /// Klass情報をもとにwix_klassテーブルからIDを取得（既存データのみ）
    async fn get_existing_klass(
        &self,
        cat1: &str,
        cat2: Option<&str>,
        cat3: Option<&str>,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        // 不正なKlass形式を検証して除外
        if self.is_invalid_klass(cat1, cat2) {
            return Ok(None);
        }
        
        // まず正確なマッチを試行（既存データを優先）
        let existing_klass = sqlx::query(
            r#"
            SELECT id FROM wix_klass 
            WHERE cat1 = $1 AND 
                  COALESCE(cat2, '') = COALESCE($2, '') AND 
                  COALESCE(cat3, '') = COALESCE($3, '')
            "#
        )
        .bind(cat1)
        .bind(cat2)
        .bind(cat3)
        .fetch_optional(self.pool.as_ref())
        .await?;

        if let Some(row) = existing_klass {
            let id: i64 = row.get("id");
            return Ok(Some(id));
        }
        
        // 正確なマッチがない場合、5バイト制限で切り詰めて再試行
        let cat1_truncated = if cat1.len() > 5 {
            cat1.chars()
                .scan(0, |acc, c| {
                    let char_len = c.len_utf8();
                    if *acc + char_len <= 5 {
                        *acc += char_len;
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect::<String>()
        } else {
            cat1.to_string()
        };
        
        let cat2_truncated = cat2.map(|s| {
            if s.len() > 5 {
                s.chars()
                    .scan(0, |acc, c| {
                        let char_len = c.len_utf8();
                        if *acc + char_len <= 5 {
                            *acc += char_len;
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .collect::<String>()
            } else {
                s.to_string()
            }
        });
        
        let cat3_truncated = cat3.map(|s| {
            if s.len() > 5 {
                s.chars()
                    .scan(0, |acc, c| {
                        let char_len = c.len_utf8();
                        if *acc + char_len <= 5 {
                            *acc += char_len;
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .collect::<String>()
            } else {
                s.to_string()
            }
        });
        
        // 切り詰められた値で検索
        let truncated_klass = sqlx::query(
            r#"
            SELECT id FROM wix_klass 
            WHERE cat1 = $1 AND 
                  COALESCE(cat2, '') = COALESCE($2, '') AND 
                  COALESCE(cat3, '') = COALESCE($3, '')
            "#
        )
        .bind(&cat1_truncated)
        .bind(cat2_truncated.as_deref())
        .bind(cat3_truncated.as_deref())
        .fetch_optional(self.pool.as_ref())
        .await?;

        if let Some(row) = truncated_klass {
            let id: i64 = row.get("id");
            Ok(Some(id))
        } else {
            // 既存データにない場合は None を返す（新規作成しない）
            Ok(None)
        }
    }

    /// wix_card_klassテーブルにリレーションを追加（重複回避）
    async fn assign_klass_to_card(
        &self,
        card_id: i64,
        klass_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO wix_card_klass (card_id, klass_id)
            VALUES ($1, $2)
            ON CONFLICT (card_id, klass_id) DO NOTHING
            "#
        )
        .bind(card_id)
        .bind(klass_id)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    pub async fn save_card(
        &self,
        create_card: CreateCard,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // フィールド長チェック（デバッグ用）
        // if let Some(ref power) = create_card.power {
        //     if power.len() > 5 {
        //         // eprintln!("WARNING: Power field too long: '{}' (length: {})", power, power.len());
        //     }
        // }
        // if let Some(ref cost) = create_card.cost {
        //     if cost.len() > 16 {
        //         // eprintln!("WARNING: Cost field too long: '{}' (length: {})", cost, cost.len());
        //     }
        // }
        // if let Some(ref story) = create_card.story {
        //     if story.len() > 16 {
        //         eprintln!("WARNING: Story field too long: '{}' (length: {})", story, story.len());
        //     }
        // }
        // if let Some(ref rarity) = create_card.rarity {
        //     if rarity.len() > 8 {
        //         eprintln!("WARNING: Rarity field too long: '{}' (length: {})", rarity, rarity.len());
        //     }
        // }
        // // デバッグ：全てのフィールド値をログ出力
        // eprintln!("DEBUG: All field values for card {}:", create_card.code);
        // eprintln!("  name: '{}' ({} bytes)", create_card.name, create_card.name.len());
        // eprintln!("  code: '{}' ({} bytes)", create_card.code, create_card.code.len());
        // eprintln!("  pronunciation: '{}' ({} bytes)", create_card.pronunciation, create_card.pronunciation.len());
        // eprintln!("  color: {}", create_card.color);
        // if let Some(ref cost) = create_card.cost {
        //     eprintln!("  cost: '{}' ({} bytes)", cost, cost.len());
        // } else {
        //     eprintln!("  cost: None");
        // }
        // eprintln!("  level: {:?}", create_card.level);
        // eprintln!("  limit: {:?}", create_card.limit);
        // eprintln!("  limit_ex: {:?}", create_card.limit_ex);
        // eprintln!("  product: {}", create_card.product);
        // eprintln!("  card_type: {}", create_card.card_type);
        // if let Some(ref power) = create_card.power {
        //     eprintln!("  power: '{}' ({} bytes)", power, power.len());
        // } else {
        //     eprintln!("  power: None");
        // }
        // eprintln!("  has_burst: {}", create_card.has_burst);
        // if let Some(ref skill_text) = create_card.skill_text {
        //     eprintln!("  skill_text: '{}' ({} bytes)", skill_text, skill_text.len());
        // } else {
        //     eprintln!("  skill_text: None");
        // }
        // if let Some(ref burst_text) = create_card.burst_text {
        //     eprintln!("  burst_text: '{}' ({} bytes)", burst_text, burst_text.len());
        // } else {
        //     eprintln!("  burst_text: None");
        // }
        // eprintln!("  format: {}", create_card.format);
        // if let Some(ref story) = create_card.story {
        //     eprintln!("  story: '{}' ({} bytes)", story, story.len());
        // } else {
        //     eprintln!("  story: None");
        // }
        // if let Some(ref rarity) = create_card.rarity {
        //     eprintln!("  rarity: '{}' ({} bytes)", rarity, rarity.len());
        // } else {
        //     eprintln!("  rarity: None");
        // }
        // eprintln!("  timing: {:?}", create_card.timing);
        // if let Some(ref url) = create_card.url {
        //     eprintln!("  url: '{}' ({} bytes)", url, url.len());
        // } else {
        //     eprintln!("  url: None");
        // }
        // eprintln!("  feature_bits1: {}", create_card.feature_bits1);
        // eprintln!("  feature_bits2: {}", create_card.feature_bits2);
        // if let Some(ref ex1) = create_card.ex1 {
        //     eprintln!("  ex1: '{}' ({} bytes)", ex1, ex1.len());
        // } else {
        //     eprintln!("  ex1: None");
        // }
        
        // デバッグ：制限違反をチェック（修正後の値で）
        let mut violations = Vec::new();
        
        if create_card.name.len() > 256 {
            violations.push(format!("name too long: {} > 256", create_card.name.len()));
        }
        if create_card.code.len() > 16 {
            violations.push(format!("code too long: {} > 16", create_card.code.len()));
        }
        if create_card.pronunciation.len() > 128 {
            violations.push(format!("pronunciation too long: {} > 128", create_card.pronunciation.len()));
        }
        if let Some(ref cost) = create_card.cost {
            if cost.len() > 16 {
                violations.push(format!("cost too long: {} > 16", cost.len()));
            }
        }
        if let Some(ref power) = create_card.power {
            if power.len() > 5 {
                violations.push(format!("power too long: {} > 5", power.len()));
            }
        }
        if let Some(ref story) = create_card.story {
            if story.len() > 16 {
                violations.push(format!("story too long: {} > 16", story.len()));
            }
        }
        if let Some(ref rarity) = create_card.rarity {
            if rarity.len() > 8 {
                violations.push(format!("rarity too long: {} > 8", rarity.len()));
            }
        }
        if let Some(ref url) = create_card.url {
            if url.len() > 200 {
                violations.push(format!("url too long: {} > 200", url.len()));
            }
        }
        if let Some(ref ex1) = create_card.ex1 {
            if ex1.len() > 256 {
                violations.push(format!("ex1 too long: {} > 256", ex1.len()));
            }
        }
        
        if !violations.is_empty() {
            return Err(format!("Field length violations: {}", violations.join(", ")).into());
        }

        let result = sqlx::query(
            r#"
            INSERT INTO wix_card (
                name, code, pronunciation, color, cost, level, "limit", limit_ex,
                product, card_type, power, has_burst, skill_text, burst_text,
                "format", story, rarity, timing, url, feature_bits1, feature_bits2, burst_bits, ex1
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            ON CONFLICT (code) DO UPDATE SET
                name = EXCLUDED.name,
                pronunciation = EXCLUDED.pronunciation,
                color = EXCLUDED.color,
                cost = EXCLUDED.cost,
                level = EXCLUDED.level,
                "limit" = EXCLUDED."limit",
                limit_ex = EXCLUDED.limit_ex,
                product = EXCLUDED.product,
                card_type = EXCLUDED.card_type,
                power = EXCLUDED.power,
                has_burst = EXCLUDED.has_burst,
                skill_text = EXCLUDED.skill_text,
                burst_text = EXCLUDED.burst_text,
                "format" = EXCLUDED."format",
                story = EXCLUDED.story,
                rarity = EXCLUDED.rarity,
                timing = EXCLUDED.timing,
                url = EXCLUDED.url,
                feature_bits1 = EXCLUDED.feature_bits1,
                feature_bits2 = EXCLUDED.feature_bits2,
                burst_bits = EXCLUDED.burst_bits,
                ex1 = EXCLUDED.ex1
            RETURNING id
            "#
        )
        .bind(&create_card.name)
        .bind(&create_card.code)
        .bind(&create_card.pronunciation)
        .bind(&create_card.color)
        .bind(&create_card.cost)
        .bind(&create_card.level)
        .bind(&create_card.limit)
        .bind(&create_card.limit_ex)
        .bind(&create_card.product)
        .bind(&create_card.card_type)
        .bind(&create_card.power)
        .bind(&create_card.has_burst)
        .bind(&create_card.skill_text)
        .bind(&create_card.burst_text)
        .bind(&create_card.format)
        .bind(&create_card.story)
        .bind(&create_card.rarity)
        .bind(&create_card.timing)
        .bind(&create_card.url)
        .bind(&create_card.feature_bits1)
        .bind(&create_card.feature_bits2)
        .bind(&create_card.burst_bits)
        .bind(&create_card.ex1)
        .fetch_one(self.pool.as_ref())
        .await?;

        let id: i64 = result.get("id");
        Ok(id)
    }

    pub async fn save_card_with_klass(
        &self,
        create_card_with_klass: CreateCardWithKlass,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // まずCardを保存
        let card_id = self.save_card(create_card_with_klass.create_card).await?;

        // 検出されたKlassを処理
        for (cat1, cat2, cat3) in create_card_with_klass.detected_klasses {
            if let Some(klass_id) = self.get_existing_klass(
                &cat1,
                cat2.as_deref(),
                cat3.as_deref(),
            ).await? {
                self.assign_klass_to_card(card_id, klass_id).await?;
            }
            // 既存のKlassが見つからない場合はスキップ（エラーにしない）
        }

        Ok(card_id)
    }
}

/// RawCardを解析してDBに保存する
pub async fn analyze_and_save_card(
    raw_card: &RawCardDb,
    pool: &Pool<Postgres>,
) -> Result<i64, Box<dyn std::error::Error>> {
    analyze_and_save_card_with_product_id(raw_card, None, pool).await
}

/// RawCardを解析してDBに保存する (product_id指定版)
pub async fn analyze_and_save_card_with_product_id(
    raw_card: &RawCardDb,
    product_id: Option<i64>,
    pool: &Pool<Postgres>,
) -> Result<i64, Box<dyn std::error::Error>> {
    let analyzer = SimpleRawCardAnalyzer::new();

    // RawCardを解析
    let create_card_with_klass = analyzer
        .analyze_with_product_id(raw_card, product_id)
        .await?;

    // DBに保存（Klass情報も含む）
    let card_repo = CardRepository::new(Arc::new(pool.clone()));
    let card_id = card_repo.save_card_with_klass(create_card_with_klass).await?;

    // RawCardを解析済みにマーク
    sqlx::query(
        "UPDATE wix_rawcard SET is_analyzed = true, last_analyzed_at = NOW() WHERE id = $1",
    )
    .bind(raw_card.id)
    .execute(pool)
    .await?;

    Ok(card_id)
}

/// 複数のRawCardを一括解析・保存
pub async fn analyze_raw_cards_batch(
    raw_cards: Vec<RawCardDb>,
    pool: &Pool<Postgres>,
) -> Vec<Result<i64, Box<dyn std::error::Error>>> {
    let mut results = Vec::new();

    for raw_card in raw_cards {
        let result = analyze_and_save_card(&raw_card, pool).await;
        results.push(result);
    }

    results
}

/// 複数のRawCardWithProductを一括解析・保存
pub async fn analyze_raw_cards_with_product_batch(
    raw_cards: Vec<RawCardWithProduct>,
    pool: &Pool<Postgres>,
) -> Vec<Result<i64, Box<dyn std::error::Error>>> {
    let mut results = Vec::new();

    for raw_card_with_product in raw_cards {
        let raw_card = raw_card_with_product.to_raw_card_db();
        let result = analyze_and_save_card_with_product_id(
            &raw_card,
            raw_card_with_product.product_id,
            pool,
        )
        .await;
        results.push(result);
    }

    results
}
