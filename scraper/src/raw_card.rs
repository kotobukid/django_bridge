use models::gen::django_models::CreateRawCard;
use scraper_html::{Html, Selector as ScraperSelector};
use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;

/// RawCard 作成とテキスト抽出のためのサービス
pub struct RawCardService {
    skill_selector: ScraperSelector,
    life_burst_selector: ScraperSelector,
}

impl RawCardService {
    /// 新しいRawCardServiceを作成
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            // スキルテキストのセレクタ（より汎用的なセレクタ）
            skill_selector: ScraperSelector::parse("td")?,
            // ライフバーストテキストのセレクタ
            life_burst_selector: ScraperSelector::parse("td")?,
        })
    }

    /// HTMLからCreateRawCardを作成
    pub fn create_raw_card_from_html(
        &self,
        card_number: String,
        name: String,
        source_url: String,
        raw_html: String,
    ) -> Result<CreateRawCard, Box<dyn std::error::Error>> {
        // HTMLをパース
        let document = Html::parse_document(&raw_html);

        // スキルテキストを抽出
        let skill_text = self.extract_skill_text(&document);

        // ライフバーストテキストを抽出
        let life_burst_text = self.extract_life_burst_text(&document);

        // 抽出したテキスト部分を除去したHTMLを作成
        let cleaned_html = self.remove_extracted_text(&raw_html, &skill_text, &life_burst_text)?;

        Ok(CreateRawCard {
            card_number,
            name,
            raw_html: cleaned_html,
            skill_text,
            life_burst_text,
            source_url,
            scraped_at: chrono::Utc::now(),
            last_analyzed_at: None,
            is_analyzed: false,
            analysis_error: String::new(),
        })
    }

    /// スキルテキストを抽出
    fn extract_skill_text(&self, document: &Html) -> String {
        // まず.cardSkillクラスから抽出を試す（詳細ページ用）
        if let Ok(card_skill_selector) = ScraperSelector::parse(".cardSkill") {
            if let Some(element) = document.select(&card_skill_selector).next() {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join("\n")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    return text;
                }
            }
        }

        // テーブル行を検索してスキルテキストを見つける
        if let Ok(tr_selector) = ScraperSelector::parse("tr") {
            for tr in document.select(&tr_selector) {
                // th要素を探す
                if let Ok(th_selector) = ScraperSelector::parse("th") {
                    if let Some(th) = tr.select(&th_selector).next() {
                        let th_text = th.text().collect::<String>();
                        if th_text.contains("テキスト") && !th_text.contains("ライフバースト")
                        {
                            // 対応するtd要素を取得
                            if let Ok(td_selector) = ScraperSelector::parse("td") {
                                if let Some(td) = tr.select(&td_selector).next() {
                                    return td
                                        .text()
                                        .collect::<Vec<_>>()
                                        .join(" ")
                                        .trim()
                                        .to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        String::new()
    }

    /// ライフバーストテキストを抽出
    fn extract_life_burst_text(&self, document: &Html) -> String {
        // HTMLからライフバーストアイコンを含むテキストを探す
        let html_string = document.html();

        // ライフバーストアイコンがある場合、その後のテキストを抽出
        if html_string.contains("icon_txt_burst.png") {
            // ライフバーストアイコンを含む行を抽出
            if let Ok(img_selector) = ScraperSelector::parse("img[alt*='ライフバースト']") {
                for img in document.select(&img_selector) {
                    // 親要素から同じdiv内のテキストを取得
                    if let Some(parent) = img.parent() {
                        // ElementRefに変換してからテキストを取得
                        if let Some(parent_element) = scraper_html::ElementRef::wrap(parent) {
                            let text = parent_element
                                .text()
                                .collect::<Vec<_>>()
                                .join(" ")
                                .trim()
                                .to_string();
                            // ライフバーストアイコンの後のテキストを抽出（：の後）
                            if let Some(colon_pos) = text.find('：') {
                                let burst_text = text[colon_pos + 3..].trim(); // 3バイトは '：' の文字
                                if !burst_text.is_empty() {
                                    return burst_text.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }

        // テーブル行を検索してライフバーストテキストを見つける（フォールバック）
        if let Ok(tr_selector) = ScraperSelector::parse("tr") {
            for tr in document.select(&tr_selector) {
                if let Ok(th_selector) = ScraperSelector::parse("th") {
                    if let Some(th) = tr.select(&th_selector).next() {
                        let th_text = th.text().collect::<String>();
                        if th_text.contains("ライフバースト") {
                            if let Ok(td_selector) = ScraperSelector::parse("td") {
                                if let Some(td) = tr.select(&td_selector).next() {
                                    return td
                                        .text()
                                        .collect::<Vec<_>>()
                                        .join(" ")
                                        .trim()
                                        .to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        String::new()
    }

    /// 抽出したテキスト部分を除去したHTMLを作成
    ///
    /// 注意: これは簡易的な実装です。より正確な除去が必要な場合は
    /// HTMLパーサーを使って該当要素を削除する方が良いでしょう。
    fn remove_extracted_text(
        &self,
        html: &str,
        skill_text: &str,
        life_burst_text: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut cleaned = html.to_string();

        // スキルテキストを除去（空でない場合のみ）
        if !skill_text.is_empty() {
            // より正確な除去のため、テキストが含まれる行全体を削除
            cleaned = cleaned.replace(skill_text, "[SKILL_TEXT_EXTRACTED]");
        }

        // ライフバーストテキストを除去（空でない場合のみ）
        if !life_burst_text.is_empty() {
            cleaned = cleaned.replace(life_burst_text, "[LIFE_BURST_TEXT_EXTRACTED]");
        }

        Ok(cleaned)
    }

    /// RawCardをデータベースに保存
    pub async fn save_raw_card(
        &self,
        pool: Arc<Pool<Postgres>>,
        create_raw_card: CreateRawCard,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        self.save_raw_card_with_product(pool, create_raw_card, None)
            .await
    }

    /// RawCardをデータベースに保存 (product_id指定版)
    pub async fn save_raw_card_with_product(
        &self,
        pool: Arc<Pool<Postgres>>,
        create_raw_card: CreateRawCard,
        product_id: Option<i64>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let result = sqlx::query(
            r#"
            INSERT INTO wix_rawcard (
                card_number, name, raw_html, skill_text, 
                life_burst_text, source_url, scraped_at, last_analyzed_at, 
                is_analyzed, analysis_error, product_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (card_number) 
            DO UPDATE SET
                name = EXCLUDED.name,
                raw_html = EXCLUDED.raw_html,
                skill_text = EXCLUDED.skill_text,
                life_burst_text = EXCLUDED.life_burst_text,
                source_url = EXCLUDED.source_url,
                scraped_at = EXCLUDED.scraped_at,
                last_analyzed_at = EXCLUDED.last_analyzed_at,
                is_analyzed = EXCLUDED.is_analyzed,
                analysis_error = EXCLUDED.analysis_error,
                product_id = EXCLUDED.product_id
            RETURNING id
            "#,
        )
        .bind(&create_raw_card.card_number)
        .bind(&create_raw_card.name)
        .bind(&create_raw_card.raw_html)
        .bind(&create_raw_card.skill_text)
        .bind(&create_raw_card.life_burst_text)
        .bind(&create_raw_card.source_url)
        .bind(&create_raw_card.scraped_at)
        .bind(&create_raw_card.last_analyzed_at)
        .bind(&create_raw_card.is_analyzed)
        .bind(&create_raw_card.analysis_error)
        .bind(product_id)
        .fetch_one(pool.as_ref())
        .await?;

        let id: i64 = result.get("id");
        Ok(id)
    }

    /// カード名をHTMLから抽出（カード番号が分からない場合用）
    pub fn extract_card_name_from_html(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);

        // まず.cardNameクラスから抽出を試す（詳細ページ用）
        if let Ok(card_name_selector) = ScraperSelector::parse(".cardName") {
            if let Some(element) = document.select(&card_name_selector).next() {
                // <br>タグがある場合の処理
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }

        // h1タグから抽出を試す
        if let Ok(h1_selector) = ScraperSelector::parse("h1") {
            if let Some(h1) = document.select(&h1_selector).next() {
                let h1_text = h1.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !h1_text.is_empty() {
                    return Some(h1_text);
                }
            }
        }

        // テーブルからカード名を探す
        if let Ok(dt_selector) = ScraperSelector::parse("dt") {
            for dt in document.select(&dt_selector) {
                let dt_text = dt.text().collect::<String>();
                if dt_text.contains("カード名") {
                    // dtの次のdd要素を探す（より簡単な方法）
                    if let Ok(dd_selector) = ScraperSelector::parse("dd") {
                        if let Some(parent) = dt.parent() {
                            if let Some(parent_element) = scraper_html::ElementRef::wrap(parent) {
                                // 同じ親要素内の次のdd要素を探す
                                for dd in parent_element.select(&dd_selector) {
                                    let dd_text =
                                        dd.text().collect::<Vec<_>>().join(" ").trim().to_string();
                                    if !dd_text.is_empty() {
                                        return Some(dd_text);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_card_service_creation() {
        let service = RawCardService::new();
        assert!(service.is_ok());
    }

    #[test]
    fn test_extract_skill_text_empty() {
        let service = RawCardService::new().unwrap();
        let html = "<html><body></body></html>";
        let document = Html::parse_document(html);

        let skill_text = service.extract_skill_text(&document);
        assert_eq!(skill_text, "");
    }

    #[test]
    fn test_remove_extracted_text() {
        let service = RawCardService::new().unwrap();
        let html = "<html><body>Some skill text here and some other content</body></html>";
        let skill_text = "Some skill text here";
        let life_burst_text = "";

        let result = service.remove_extracted_text(html, skill_text, life_burst_text);
        assert!(result.is_ok());

        let cleaned = result.unwrap();
        assert!(cleaned.contains("[SKILL_TEXT_EXTRACTED]"));
        assert!(!cleaned.contains("Some skill text here"));
    }
}
