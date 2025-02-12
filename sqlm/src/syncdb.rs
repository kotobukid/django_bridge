use rustpython_parser::lexer::lex;
use rustpython_parser::Tok;
use rustpython_parser_core::mode::Mode;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn generate_struct_from_python(
    struct_name: &str,
    python_code: &str,
    use_crate: &mut UseCrate,
) -> String {
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
        "#[allow(dead_code)]\n#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]\npub struct {} {{\n",
        format!("{}Db", struct_name)
    );

    rust_struct.push_str("    /// Primary Key\n    pub id: i64,\n");

    for (field_name, field_type, is_nullable, default_value, max_length) in fields {
        let rust_type = match field_type.as_str() {
            "AutoField" => "u32",
            "BigAutoField" => "u64",
            "BigIntegerField" => "i64",
            "BinaryField" => "Vec<u8>",
            "BooleanField" => "bool",
            "CharField" => "String",
            "DateField" => "NaiveDate",
            "DateTimeField" => "DateTime<Utc>",
            "DecimalField" => "rust_decimal::Decimal",
            "DurationField" => "chrono::Duration",
            "EmailField" => "String",
            "FileField" => "String", // filepath
            "FilePathField" => "String",
            "FloatField" => "f64",
            "GeneratedField" => "String", // フォールバック
            "GenericIPAddressField" => "std::net::IpAddr",
            "ImageField" => "String", // FileFieldと同じ扱い
            "IntegerField" => "i32",
            "JSONField" => "Value",
            "PositiveBigIntegerField" => "u64", // 0 to 9223372036854775807
            "PositiveIntegerField" => "u32",    // 0 to 2147483647
            "PositiveSmallIntegerField" => "u16", // 0 to 32767
            "SlugField" => "String",            // ascii only?
            "SmallAutoField" => "u16",          //  1 to 32767
            "SmallIntegerField" => "i16",       // -32768 to 32767
            "TextField" => "String",
            "TimeField" => "chrono::NaiveTime",
            "URLField" => "String",

            // relationships
            "ForeignKey" => "u32",
            "ManyToManyField" => "u32",
            "OneToOneField" => "u32",
            "OneToManyField" => "u32",
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

    if python_code.contains("BinaryField") {
        use_crate.use_serde_json = true; // BinaryFieldで`serde_json`が必要と仮定
    }
    if python_code.contains("DecimalField") {
        use_crate.use_rust_decimal = true; // DecimalFieldで`rust_decimal`が必要
    }
    if python_code.contains("JSONField") {
        use_crate.use_serde_json = true;
    }
    if python_code.contains("GenericIPAddressField") {
        use_crate.use_std_net = true;
    }
    if python_code.contains("TimeField") {
        use_crate.use_chrono = true;
        use_crate.use_chrono_naive_time = true;
    }
    if python_code.contains("DurationField") {
        use_crate.use_chrono = true;
        use_crate.use_chrono_duration = true;
    }
    if python_code.contains("DateField") {
        use_crate.use_chrono = true;
        use_crate.use_chrono_naive_date = true;
    }
    if python_code.contains("DateTimeField") {
        use_crate.use_chrono = true;
        use_crate.use_chrono_datetimetz = true;
    }

    rust_struct.push_str("}\n");
    rust_struct
}

struct UseCrate {
    use_serde: bool,
    use_chrono: bool,
    use_chrono_naive_date: bool,
    use_chrono_naive_time: bool,
    use_chrono_duration: bool,
    use_chrono_datetimetz: bool,
    use_rust_decimal: bool,
    use_serde_json: bool,
    use_std_net: bool,
    use_rust_decimal_macros: bool,
    use_rust_decimal_ops: bool,
}
impl UseCrate {
    fn new() -> Self {
        Self {
            use_serde: false,
            use_chrono: false,
            use_chrono_naive_date: false,
            use_chrono_naive_time: false,
            use_chrono_duration: false,
            use_chrono_datetimetz: false,
            use_rust_decimal: false,
            use_serde_json: false,
            use_std_net: false,
            use_rust_decimal_macros: false,
            use_rust_decimal_ops: false,
        }
    }

    fn write_use_statements(&self, file: &mut fs::File) {
        if self.use_chrono {
            let mut modules = vec![];
            if self.use_chrono_naive_date {
                modules.push("NaiveDate");
            }
            if self.use_chrono_naive_time {
                modules.push("NaiveTime");
            }
            if self.use_chrono_duration {
                modules.push("Duration");
            }
            if self.use_chrono_datetimetz {
                modules.push("DateTime, Utc");
            }
            writeln!(file, "use chrono::{{{}}};", modules.join(", ")).unwrap();
        }
        if self.use_serde {
            writeln!(file, "use serde::{{Serialize, Deserialize}};").unwrap();
        }
        if self.use_serde_json {
            writeln!(file, "use serde_json::Value;").unwrap();
        }
        if self.use_rust_decimal {
            writeln!(file, "use rust_decimal::Decimal;").unwrap();
        }
        if self.use_std_net {
            writeln!(file, "use std::net::IpAddr;").unwrap();
        }
        if self.use_rust_decimal_macros {
            writeln!(file, "use rust_decimal_macros;").unwrap();
        }
        if self.use_rust_decimal_ops {
            writeln!(file, "use rust_decimal_ops;").unwrap();
        }
    }
}

fn main() {
    let out_dir = "src/gen";
    fs::create_dir_all(out_dir).unwrap();

    let models = [
        ("Card", "../table_definition/wix/models.py"),
        ("Tag", "../table_definition/wix/models.py"),
    ];

    let dest_path = Path::new(out_dir).join("django_models.rs");

    fs::remove_file(&dest_path).ok();

    let mut file = OpenOptions::new()
        .create(true) // ファイルがなければ作成
        .append(true) // 追記モード
        .open(&dest_path)
        .unwrap();

    let mut use_crate = UseCrate::new();
    use_crate.use_serde = true;

    // モデル定義を収集する
    let struct_defs: Vec<String> = models
        .iter()
        .map(|(struct_name, file_path)| {
            let python_code = fs::read_to_string(file_path).unwrap();

            // 構造体生成コードと依存クレート解析
            generate_struct_from_python(struct_name, &python_code, &mut use_crate)
        })
        .collect(); // Vec<String> に変換

    // 必要なuse文を冒頭に書き込み
    use_crate.write_use_statements(&mut file);

    // 収集したモデル情報
    file.write_all(struct_defs.join("\n").as_bytes()).unwrap();
}
