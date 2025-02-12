use rustpython_parser::lexer::lex;
use rustpython_parser::Tok;
use rustpython_parser_core::mode::Mode;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn generate_struct_from_python(struct_name: &str, python_code: &str) -> String {
    // トークンを収集
    let tokens = lex(python_code, Mode::Module)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to tokenize Python code");

    let mut fields = vec![];
    let mut in_class_def = false;
    let mut current_class_name = String::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i].0 {
            // クラスの検出
            Tok::Class => {
                if i + 1 < tokens.len() {
                    if let Tok::Name { name: class_name } = &tokens[i + 1].0 {
                        current_class_name = class_name.clone();
                        in_class_def = true;
                        println!("Found class: {}", current_class_name);
                    }
                }
                i += 1;
                continue;
            }
            // フィールド定義の検出
            Tok::Name { name } if in_class_def && current_class_name == struct_name => {
                if i + 4 < tokens.len() {
                    if let (Tok::Equal, _) = &tokens[i + 1] {
                        if let (Tok::Name { name: models_name }, _) = &tokens[i + 2] {
                            if models_name == "models" {
                                if let (Tok::Dot, _) = &tokens[i + 3] {
                                    if let (Tok::Name { name: field_type }, _) = &tokens[i + 4] {
                                        if field_type.ends_with("Field") {
                                            println!(
                                                "Found field: {} of type {}",
                                                name, field_type
                                            );

                                            // 属性を解析
                                            let mut is_nullable = false;
                                            let mut default_value = None;
                                            let mut max_length = None;

                                            let mut j = i + 5;
                                            while j < tokens.len() {
                                                match &tokens[j].0 {
                                                    Tok::Newline => break,
                                                    // null属性
                                                    Tok::Name { name: kw } if kw == "null" => {
                                                        if let Some((Tok::Equal, _)) =
                                                            tokens.get(j + 1)
                                                        {
                                                            if let Some((Tok::True, _)) =
                                                                tokens.get(j + 2)
                                                            {
                                                                is_nullable = true;
                                                            }
                                                        }
                                                    }
                                                    // default属性
                                                    Tok::Name { name: kw } if kw == "default" => {
                                                        if let Some((Tok::Equal, _)) =
                                                            tokens.get(j + 1)
                                                        {
                                                            if let Some((
                                                                            Tok::String { value, .. },
                                                                            _,
                                                                        )) = tokens.get(j + 2)
                                                            {
                                                                default_value = Some(value.clone());
                                                            }
                                                        }
                                                    }
                                                    // max_length属性
                                                    Tok::Name { name: kw }
                                                    if kw == "max_length" =>
                                                        {
                                                            if let Some((Tok::Equal, _)) =
                                                                tokens.get(j + 1)
                                                            {
                                                                if let Some((
                                                                                Tok::Int { value, .. },
                                                                                _,
                                                                            )) = tokens.get(j + 2)
                                                                {
                                                                    // BigIntをStringに変換して保存
                                                                    max_length =
                                                                        Some(value.to_string());
                                                                }
                                                            }
                                                        }
                                                    _ => {}
                                                }
                                                j += 1;
                                            }

                                            // フィールド情報を追加
                                            fields.push((
                                                name.clone(),
                                                field_type.clone(),
                                                is_nullable,
                                                default_value,
                                                max_length,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Rustの構造体定義を生成
    let mut rust_struct = format!(
        "#[derive(sqlx::FromRow, Debug, Clone)]\npub struct {} {{\n",
        format!("{}Db", struct_name)
    );

    rust_struct.push_str("    /// Primary Key\n    pub id: i64,\n");

    for (field_name, field_type, is_nullable, default_value, max_length) in fields {
        let rust_type = match field_type.as_str() {
            "CharField" => "String",
            "DateTimeField" => "chrono::DateTime<chrono::Utc>",
            "IntegerField" => "i32",
            "FloatField" => "f64",
            "BooleanField" => "bool",
            "TextField" => "String",
            "EmailField" => "String",
            "DecimalField" => "rust_decimal::Decimal",
            _ => "String", // フォールバック
        };

        // null許容型の場合Rustの型をOptionでラップ
        let rust_field_type = if is_nullable {
            format!("Option<{}>", rust_type)
        } else {
            rust_type.to_string()
        };

        // コメント生成（default値とmax_lengthを含める）
        if default_value.is_some() || max_length.is_some() {
            rust_struct.push_str("    /// ");
            if let Some(default) = &default_value {
                rust_struct.push_str(&format!("Default: {}, ", default));
            }
            if let Some(length) = &max_length {
                rust_struct.push_str(&format!("Max length: {}", length));
            }
            rust_struct.push_str("\n");
        }

        // フィールド定義追加
        rust_struct.push_str(&format!("    pub {}: {},\n", field_name, rust_field_type));
    }

    rust_struct.push_str("}\n");
    rust_struct
}

fn main() {
    let out_dir = "src/gen";
    fs::create_dir_all(out_dir).unwrap();

    let models = [
        ("Card", "../table_definition/wix/models.py"),
        ("Tag", "../table_definition/wix/models.py"),
    ];

    let dest_path = Path::new(out_dir).join("django_models.rs");

    fs::remove_file(&dest_path).unwrap();

    let mut file = OpenOptions::new()
        .create(true) // ファイルがなければ作成
        .append(true) // 追記モード
        .open(&dest_path)
        .unwrap();

    writeln!(file, "use sqlx;\nuse chrono;\n").unwrap();

    for (struct_name, file_path) in models {
        let python_code = fs::read_to_string(file_path).unwrap();

        writeln!(
            file,
            "\n{}",
            generate_struct_from_python(struct_name, &*python_code)
        )
            .unwrap();
    }
}
