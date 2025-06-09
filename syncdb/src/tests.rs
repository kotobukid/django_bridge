#[cfg(test)]
mod tests {
    use crate::*;
    use rustpython_parser::{StringKind, Tok};

    #[test]
    fn test_first_upper() {
        assert_eq!(first_upper(""), "");
        assert_eq!(first_upper("a"), "A");
        assert_eq!(first_upper("hello"), "Hello");
        assert_eq!(first_upper("Hello"), "Hello");
        assert_eq!(first_upper("123"), "123");
        assert_eq!(first_upper("_test"), "_test");

        // パフォーマンス最適化のテスト - すでに大文字の場合は借用を返す
        let already_upper = "Hello";
        match first_upper(already_upper) {
            std::borrow::Cow::Borrowed(s) => assert_eq!(s, already_upper),
            std::borrow::Cow::Owned(_) => panic!("Expected borrowed value"),
        }
    }

    #[test]
    fn test_map_django_field_to_rust_type() {
        // 基本的なフィールドタイプのテスト
        assert_eq!(
            map_django_field_to_rust_type("CharField"),
            DjangoFieldType::Valid("String")
        );
        assert_eq!(
            map_django_field_to_rust_type("IntegerField"),
            DjangoFieldType::Valid("i32")
        );
        assert_eq!(
            map_django_field_to_rust_type("BooleanField"),
            DjangoFieldType::Valid("bool")
        );
        assert_eq!(
            map_django_field_to_rust_type("DateField"),
            DjangoFieldType::Valid("NaiveDate")
        );
        assert_eq!(
            map_django_field_to_rust_type("DateTimeField"),
            DjangoFieldType::Valid("DateTime<Utc>")
        );
        assert_eq!(
            map_django_field_to_rust_type("DecimalField"),
            DjangoFieldType::Valid("rust_decimal::Decimal")
        );
        assert_eq!(
            map_django_field_to_rust_type("JSONField"),
            DjangoFieldType::Valid("Value")
        );

        // 数値型のテスト
        assert_eq!(
            map_django_field_to_rust_type("BigIntegerField"),
            DjangoFieldType::Valid("i64")
        );
        assert_eq!(
            map_django_field_to_rust_type("PositiveIntegerField"),
            DjangoFieldType::Valid("u32")
        );
        assert_eq!(
            map_django_field_to_rust_type("SmallIntegerField"),
            DjangoFieldType::Valid("i16")
        );

        // リレーションフィールドのテスト
        assert_eq!(
            map_django_field_to_rust_type("ForeignKey"),
            DjangoFieldType::Relation("i64")
        );
        assert_eq!(
            map_django_field_to_rust_type("OneToOneField"),
            DjangoFieldType::Relation("i64")
        );
        assert_eq!(
            map_django_field_to_rust_type("ManyToManyField"),
            DjangoFieldType::ManyToMany
        );

        // 未知のフィールドタイプのテスト
        match map_django_field_to_rust_type("UnknownField") {
            DjangoFieldType::None(ty) => assert_eq!(ty, "UnknownField"),
            _ => panic!("Expected DjangoFieldType::None for unknown field"),
        }
    }

    #[test]
    fn test_analyze_relation_field_with_string_literal() {
        // ForeignKey("Product") のケース
        let tokens = vec![
            Tok::Name {
                name: "ForeignKey".to_string(),
            },
            Tok::Lpar,
            Tok::String {
                value: "Product".to_string(),
                kind: StringKind::String,
                triple_quoted: false,
            },
            Tok::Rpar,
        ];

        let result = analyze_relation_field(tokens, "product");
        assert_eq!(result, Some("Product".to_string()));
    }

    #[test]
    fn test_analyze_relation_field_with_to_parameter() {
        // ForeignKey(to="Product") のケース
        let tokens = vec![
            Tok::Name {
                name: "ForeignKey".to_string(),
            },
            Tok::Lpar,
            Tok::Name {
                name: "to".to_string(),
            },
            Tok::Equal,
            Tok::String {
                value: "Product".to_string(),
                kind: StringKind::String,
                triple_quoted: false,
            },
            Tok::Rpar,
        ];

        let result = analyze_relation_field(tokens, "product");
        assert_eq!(result, Some("Product".to_string()));
    }

    #[test]
    fn test_analyze_relation_field_with_model_reference() {
        // ForeignKey(Product) のケース（文字列ではなく、直接モデル参照）
        let tokens = vec![
            Tok::Name {
                name: "ForeignKey".to_string(),
            },
            Tok::Lpar,
            Tok::Name {
                name: "Product".to_string(),
            },
            Tok::Rpar,
        ];

        let result = analyze_relation_field(tokens, "product");
        assert_eq!(result, Some("Product".to_string()));
    }

    #[test]
    fn test_analyze_relation_field_empty() {
        let tokens = vec![];
        let result = analyze_relation_field(tokens, "field");
        assert_eq!(result, None);
    }

    #[test]
    fn test_update_crate_requirements() {
        let mut crate_requirements = CrateRequirements::new();

        // 初期状態の確認
        assert!(!crate_requirements.use_chrono);
        assert!(!crate_requirements.use_rust_decimal);
        assert!(!crate_requirements.use_serde_json);

        // DateFieldの場合
        update_crate_requirements(&mut crate_requirements, "DateField");
        assert!(crate_requirements.use_chrono);
        assert!(crate_requirements.use_chrono_naive_date);

        // DecimalFieldの場合
        update_crate_requirements(&mut crate_requirements, "DecimalField");
        assert!(crate_requirements.use_rust_decimal);

        // JSONFieldの場合
        update_crate_requirements(&mut crate_requirements, "JSONField");
        assert!(crate_requirements.use_serde_json);
    }

    #[test]
    fn test_fields_struct_creation() {
        let fields = Fields::new(
            "name".to_string(),
            "CharField".to_string(),
            false,
            Some("default_value".to_string()),
            Some("100".to_string()),
            vec![],
        );

        assert_eq!(fields.name, "name");
        assert_eq!(fields.f_type, "CharField");
        assert_eq!(fields.is_nullable, false);
        assert_eq!(fields.default_value, Some("default_value".to_string()));
        assert_eq!(fields.max_length, Some("100".to_string()));
        assert!(fields.tokens.is_empty());
    }

    #[test]
    fn test_integration_simple_model() {
        let mut crate_requirements = CrateRequirements::new();
        let python_code = r#"
class Product(models.Model):
    name = models.CharField(max_length=100)
    price = models.IntegerField()
    is_active = models.BooleanField(default=True)
"#;

        let result =
            generate_struct_from_python("myapp", "Product", python_code, &mut crate_requirements);

        assert!(result.is_ok());
        let (db_struct, create_struct, _) = result.unwrap();

        // 基本的な構造体定義が含まれているか確認
        assert!(db_struct.contains("pub struct ProductDb"));
        assert!(db_struct.contains("pub id: i64"));
        assert!(db_struct.contains("pub name: String"));
        assert!(db_struct.contains("pub price: i32"));
        assert!(db_struct.contains("pub is_active: bool"));

        // CreateProduct構造体の確認
        assert!(create_struct.contains("pub struct CreateProduct"));
        assert!(!create_struct.contains("pub id:")); // idはCreateに含まれない
    }
}
