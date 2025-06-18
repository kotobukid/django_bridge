use models::cardtype::CardTypeDb;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CardTypeRepository {
    db: Arc<Pool<Postgres>>,
    cache: HashMap<String, i64>,
}

#[derive(Debug)]
pub struct CardTypeInfo {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub sort_asc: i32,
    pub is_primary: bool,
}

impl CardTypeRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        Self {
            db,
            cache: HashMap::new(),
        }
    }

    pub async fn create_cache(&mut self) -> Result<(), sqlx::Error> {
        let res = sqlx::query_as::<_, CardTypeDb>("SELECT * FROM wix_cardtype;")
            .fetch_all(&*self.db)
            .await?;

        res.iter().for_each(|cardtype| {
            self.cache.insert(cardtype.name.clone(), cardtype.id);
        });

        Ok(())
    }

    pub fn find_id_by_name(&self, name: &str) -> Option<i64> {
        self.cache.get(name).cloned()
    }

    pub async fn find_by_code(&self, code: &str) -> Result<i64, sqlx::Error> {
        let id: i64 = sqlx::query_scalar("SELECT id FROM wix_cardtype WHERE code = $1;")
            .bind(code)
            .fetch_one(&*self.db)
            .await?;
        Ok(id)
    }

    pub async fn get_all(&self) -> Result<Vec<CardTypeInfo>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (i64, String, String, i32, bool)>(
            "SELECT id, name, code, sort_asc, is_primary FROM wix_cardtype ORDER BY sort_asc"
        )
        .fetch_all(&*self.db)
        .await?;

        let card_types = rows
            .into_iter()
            .map(|(id, name, code, sort_asc, is_primary)| CardTypeInfo {
                id,
                name,
                code,
                sort_asc,
                is_primary,
            })
            .collect();

        Ok(card_types)
    }

    pub async fn code(&self) -> String {
        match self.get_all().await {
            Ok(card_types) => self.generate_card_type_code(card_types),
            Err(e) => {
                eprintln!("Failed to fetch card types: {}", e);
                String::new()
            }
        }
    }

    fn generate_card_type_code(&self, card_types: Vec<CardTypeInfo>) -> String {
        let primary_types: Vec<&CardTypeInfo> = card_types.iter().filter(|ct| ct.is_primary).collect();
        let extended_types: Vec<&CardTypeInfo> = card_types.iter().filter(|ct| !ct.is_primary).collect();

        format!(
            r#"// カードタイプ情報
// このファイルは static_generator によって自動生成されます

#[derive(Debug, Clone)]
pub struct CardTypeInfo {{
    pub code: &'static str,
    pub name: &'static str,
    pub sort_asc: i32,
    pub is_primary: bool,
}}

pub const CARD_TYPES: &[CardTypeInfo] = &[
{}
];

pub const PRIMARY_CARD_TYPES: &[&str] = &[
{}
];

pub const EXTENDED_CARD_TYPES: &[&str] = &[
{}
];

impl CardTypeInfo {{
    pub fn find_by_code(code: &str) -> Option<&'static CardTypeInfo> {{
        CARD_TYPES.iter().find(|ct| ct.code == code)
    }}

    pub fn is_primary_type(code: &str) -> bool {{
        PRIMARY_CARD_TYPES.contains(&code)
    }}
}}
"#,
            card_types
                .iter()
                .map(|ct| format!(
                    r#"    CardTypeInfo {{
        code: "{}",
        name: "{}",
        sort_asc: {},
        is_primary: {},
    }},"#,
                    ct.code, ct.name, ct.sort_asc, ct.is_primary
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            primary_types
                .iter()
                .map(|ct| format!(r#"    "{}""#, ct.code))
                .collect::<Vec<_>>()
                .join(",\n"),
            extended_types
                .iter()
                .map(|ct| format!(r#"    "{}""#, ct.code))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}
