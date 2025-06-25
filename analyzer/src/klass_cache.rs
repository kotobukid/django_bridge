use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;

/// Klassキャッシュ管理
pub struct KlassCache {
    cache: HashMap<(String, Option<String>, Option<String>), i64>,
}

impl KlassCache {
    /// データベースから全てのKlassデータを取得してキャッシュを構築
    pub async fn new(pool: &Pool<Postgres>) -> Result<Self, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            r#"
            SELECT id, cat1, cat2, cat3 
            FROM wix_klass
            "#
        )
        .fetch_all(pool)
        .await?;

        let mut cache = HashMap::new();
        for row in rows {
            let id: i64 = row.get("id");
            let cat1: String = row.get("cat1");
            let cat2: Option<String> = row.get("cat2");
            let cat3: Option<String> = row.get("cat3");
            
            cache.insert((cat1, cat2, cat3), id);
        }

        Ok(Self { cache })
    }

    /// キャッシュからKlass IDを取得
    pub fn get(&self, cat1: &str, cat2: Option<&str>, cat3: Option<&str>) -> Option<i64> {
        // まず完全一致を試行
        let key = (cat1.to_string(), cat2.map(|s| s.to_string()), cat3.map(|s| s.to_string()));
        if let Some(&id) = self.cache.get(&key) {
            return Some(id);
        }

        // 5バイト制限で切り詰めて再試行
        let cat1_truncated = Self::truncate_to_5_bytes(cat1);
        let cat2_truncated = cat2.map(|s| Self::truncate_to_5_bytes(s));
        let cat3_truncated = cat3.map(|s| Self::truncate_to_5_bytes(s));
        
        let truncated_key = (cat1_truncated, cat2_truncated, cat3_truncated);
        self.cache.get(&truncated_key).copied()
    }

    /// UTF-8で5バイトまでに切り詰める
    fn truncate_to_5_bytes(s: &str) -> String {
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
            .collect()
    }

    /// カード-Klassリレーションをバッチで挿入
    pub async fn batch_assign_klass_to_cards(
        pool: &Pool<Postgres>,
        assignments: Vec<(i64, i64)>, // (card_id, klass_id)
    ) -> Result<(), Box<dyn std::error::Error>> {
        if assignments.is_empty() {
            return Ok(());
        }

        // バッチサイズで分割して挿入
        const BATCH_SIZE: usize = 100;
        for chunk in assignments.chunks(BATCH_SIZE) {
            let mut query = String::from(
                "INSERT INTO wix_card_klass (card_id, klass_id) VALUES "
            );
            
            let values: Vec<String> = chunk
                .iter()
                .enumerate()
                .map(|(i, _)| format!("(${}, ${})", i * 2 + 1, i * 2 + 2))
                .collect();
            
            query.push_str(&values.join(", "));
            query.push_str(" ON CONFLICT (card_id, klass_id) DO NOTHING");

            let mut q = sqlx::query(&query);
            for (card_id, klass_id) in chunk {
                q = q.bind(card_id).bind(klass_id);
            }
            
            q.execute(pool).await?;
        }

        Ok(())
    }
}