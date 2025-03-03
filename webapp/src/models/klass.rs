pub use crate::gen::django_models::{CreateKlass, KlassDb};
use crate::new_type;
// pub use crate::models::klass::{KlassDb};

new_type!(Klass, KlassDb);

pub fn create_klass(source: &str) -> CreateKlass {
    let source = create_klass_source_from_str(source);
    CreateKlass {
        cat1: source.0,
        cat2: source.1,
        cat3: source.2,
        sort_asc: 0,
    }
}

fn create_klass_source_from_str(s: &str) -> (String, Option<String>, Option<String>) {
    // 大カテゴリーと小カテゴリーを「：」で分割
    let parts: Vec<&str> = s.split('：').collect();

    // 大カテゴリー（必須）
    let cat1 = match parts.get(0) {
        Some(main_category) => main_category.to_string(),
        None => String::new(), // または適切なエラーハンドリング
    };

    // 小カテゴリーの処理
    let (cat2, cat3) = if let Some(sub_categories) = parts.get(1) {
        // 小カテゴリーを「/」で分割
        let sub_parts: Vec<&str> = sub_categories.split('/').collect();

        match (sub_parts.get(0), sub_parts.get(1)) {
            (Some(sub1), Some(sub2)) => {
                (Some(sub1.trim().to_string()), Some(sub2.trim().to_string()))
            }
            (Some(sub1), None) => (Some(sub1.trim().to_string()), None),
            _ => (None, None),
        }
    } else {
        (None, None)
    };

    (cat1, cat2, cat3)
}
