use crate::raw_card_analyzer::{AnalysisError, RawCardAnalyzer, to_half};
use chrono::{DateTime, Utc};
use models::card::{Card, CreateCard};
use models::r#gen::django_models::RawCardDb;
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use feature::{create_detect_patterns, CardFeature};
use feature::feature::HashSetToBits;
use std::collections::HashSet;
use color::convert_cost;

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
        let mut dd_elements = Vec::new();

        // <dt>カード種類</dt> または <dt>種類</dt> 以降の<dd>要素を抽出
        let start_patterns = ["<dt>カード種類</dt>", "<dt>種類</dt>"];
        let mut start_index = None;

        for pattern in &start_patterns {
            if let Some(index) = html.find(pattern) {
                start_index = Some(index);
                break;
            }
        }

        if let Some(start) = start_index {
            let mut current_pos = start;

            // <dd>要素を順番に収集
            while let Some(dd_start) = html[current_pos..].find("<dd>") {
                let absolute_dd_start = current_pos + dd_start;
                if let Some(dd_end) = html[absolute_dd_start..].find("</dd>") {
                    let absolute_dd_end = absolute_dd_start + dd_end;
                    let dd_content = &html[absolute_dd_start + 4..absolute_dd_end];
                    dd_elements.push(dd_content.to_string());
                    current_pos = absolute_dd_end + 5; // "</dd>"の後
                } else {
                    break;
                }
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

        // シグニまたはクラフトカードかどうかをチェック
        let card_type = &dd_elements[0];
        let is_signi_or_craft = card_type.contains("シグニ") || card_type.contains("クラフト");

        if is_signi_or_craft && dd_elements.len() > 7 {
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
    pub fn detect_story_from_html(&self, html: &str) -> Option<String> {
        let dd_elements = self.extract_dd_elements(html);

        if dd_elements.len() > 11 {
            let story_html = &dd_elements[11];

            // dd[11]内にdissonaアイコンがあるかチェック
            if story_html.contains("icon_txt_dissona.png") {
                Some("dissona".to_string())
            } else {
                None
            }
        } else {
            None
        }
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

    /// スキルテキストとライフバーストテキストから特徴を検出し、置換後のテキストも返す
    fn detect_features_and_replace_text(&self, skill_text: &str, life_burst_text: &str) -> (i64, i64, String, String) {
        let (replace_patterns, detect_patterns) = create_detect_patterns();
        let mut detected_features = HashSet::new();

        // 半角に変換
        let mut processed_skill_text = to_half(skill_text);
        let mut processed_burst_text = to_half(life_burst_text);

        // 置換パターンを適用して特徴を検出
        for pattern in &replace_patterns {
            // スキルテキストに対して置換を適用
            if pattern.pattern_r.is_match(&processed_skill_text) {
                processed_skill_text = pattern.pattern_r.replace_all(&processed_skill_text, pattern.replace_to).to_string();
                for feature in pattern.features_detected {
                    detected_features.insert(feature.clone());
                }
            }

            // ライフバーストテキストに対して置換を適用
            if pattern.pattern_r.is_match(&processed_burst_text) {
                processed_burst_text = pattern.pattern_r.replace_all(&processed_burst_text, pattern.replace_to).to_string();
                for feature in pattern.features_detected {
                    detected_features.insert(feature.clone());
                }
            }
        }

        // 検出パターンで特徴を検出（置換後のテキストに対して）
        let combined_text = format!("{} {}", processed_skill_text, processed_burst_text);
        for pattern in &detect_patterns {
            if pattern.pattern_r.is_match(&combined_text) {
                for feature in pattern.features_detected {
                    detected_features.insert(feature.clone());
                }
            }
        }

        // HashSetからビットに変換（feature crateの標準実装を使用）
        let (bits1, bits2) = detected_features.to_bits();

        (bits1, bits2, processed_skill_text, processed_burst_text)
    }


    pub async fn analyze_with_product_id(
        &self,
        raw_card: &RawCardDb,
        product_id: Option<i64>,
    ) -> Result<CreateCard, AnalysisError> {
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
        let story = self.detect_story_from_html(&raw_card.raw_html);

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
        let (mut feature_bits1, mut feature_bits2, replaced_skill_text, replaced_burst_text) =
            self.detect_features_and_replace_text(&raw_card.skill_text, &raw_card.life_burst_text);

        // テキスト以外からのFeature検出
        let bits = detected_feature_set.to_bits();
        feature_bits1 = feature_bits1 | bits.0;
        feature_bits2 = feature_bits2 | bits.1;

        // 基本的なCreateCardを作成
        Ok(CreateCard {
            name: to_half(&raw_card.name),
            code: to_half(&raw_card.card_number),
            pronunciation: to_half(&raw_card.name), // デフォルトで名前を使用
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
            format: 7, // デフォルトオールスター
            story,
            rarity: None,
            timing,
            url: Some(raw_card.source_url.clone()),
            feature_bits1,
            feature_bits2,
            ex1: None,
        })
    }
}

#[async_trait::async_trait]
impl RawCardAnalyzer for SimpleRawCardAnalyzer {
    async fn analyze(&self, raw_card: &RawCardDb) -> Result<CreateCard, AnalysisError> {
        self.analyze_with_product_id(raw_card, None).await
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

    pub async fn save_card(
        &self,
        create_card: CreateCard,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let result = sqlx::query(
            r#"
            INSERT INTO wix_card (
                name, code, pronunciation, color, cost, level, "limit", limit_ex,
                product, card_type, power, has_burst, skill_text, burst_text,
                "format", story, rarity, timing, url, feature_bits1, feature_bits2, ex1
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
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
        .bind(&create_card.ex1)
        .fetch_one(self.pool.as_ref())
        .await?;

        let id: i64 = result.get("id");
        Ok(id)
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
    let create_card = analyzer
        .analyze_with_product_id(raw_card, product_id)
        .await?;

    // DBに保存
    let card_repo = CardRepository::new(Arc::new(pool.clone()));
    let card_id = card_repo.save_card(create_card).await?;

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
