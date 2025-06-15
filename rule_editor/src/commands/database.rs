use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct SentenceResult {
    pub id: String,
    pub text: String,
    pub card_number: String,
    pub card_name: String,
    pub full_skill_text: String,
}

#[tauri::command]
pub async fn search_and_split(
    keyword: String,
    state: State<'_, AppState>,
) -> Result<Vec<SentenceResult>, String> {
    let pool = &state.pool;
    
    // RawCardテーブルから検索
    let query = r#"
        SELECT card_number, name, skill_text
        FROM wix_rawcard
        WHERE skill_text LIKE $1
        LIMIT 100
    "#;
    
    let search_pattern = format!("%{}%", keyword);
    let rows = sqlx::query_as::<_, (String, String, String)>(query)
        .bind(&search_pattern)
        .fetch_all(pool.as_ref())
        .await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let mut results = Vec::new();
    let mut id_counter = 0;
    
    for (card_number, card_name, skill_text) in rows {
        // skill_textを句点で分割
        let sentences: Vec<&str> = skill_text.split('。').collect();
        
        for sentence in sentences {
            // キーワードを含む文のみを抽出
            if sentence.contains(&keyword) && !sentence.trim().is_empty() {
                id_counter += 1;
                results.push(SentenceResult {
                    id: format!("{}-{}", card_number, id_counter),
                    text: format!("{}。", sentence.trim()), // 句点を復元
                    card_number: card_number.clone(),
                    card_name: card_name.clone(),
                    full_skill_text: skill_text.clone(),
                });
            }
        }
    }
    
    Ok(results)
}