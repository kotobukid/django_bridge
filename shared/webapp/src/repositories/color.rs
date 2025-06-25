use crate::repositories::StaticCodeGenerator;
use color::Color;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct ColorRepository {
    _db_connector: Arc<Pool<Postgres>>,
}

impl ColorRepository {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Self {
        Self {
            _db_connector: pool,
        }
    }
}

impl StaticCodeGenerator for ColorRepository {
    async fn code(&self) -> String {
        let lines = self.get_all_as_code().await;
        format!(
            "{}{}{}",
            ColorRepository::headline(lines.len() as i32),
            lines.join("\n"),
            ColorRepository::tail()
        )
    }

    async fn get_all_as_code(&self) -> Vec<String> {
        // Colorの全バリアントを取得してテーマカラーを生成
        let colors = vec![
            Color::White,
            Color::Blue,
            Color::Red,
            Color::Black,
            Color::Green,
            Color::Colorless,
        ];

        colors
            .into_iter()
            .map(|color| {
                let theme = color.theme();
                let color_name = match color {
                    Color::White => "White",
                    Color::Blue => "Blue",
                    Color::Red => "Red",
                    Color::Black => "Black",
                    Color::Green => "Green",
                    Color::Colorless => "Colorless",
                    _ => panic!("Unexpected color variant"),
                };

                format!(
                    r#"("{}", "{}", "{}", "{}"),"#,
                    color_name, theme.base, theme.accent, theme.light
                )
            })
            .collect()
    }

    fn headline(length: i32) -> String {
        format!(
            r#"pub type ColorThemeStatic = (&'static str, &'static str, &'static str, &'static str);
pub const COLOR_THEMES: &[ColorThemeStatic; {}] = &["#,
            length
        )
    }

    fn tail() -> &'static str {
        "];"
    }
}
