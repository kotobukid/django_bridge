use anyhow::{Context, Result};
use rustpython_parser::lexer::lex;
use rustpython_parser::Tok;
use rustpython_parser_core::mode::Mode;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncDbError {
    #[error("Failed to tokenize Python code: {0}")]
    TokenizationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Path error: {0}")]
    PathError(String),
}

#[derive(Debug, PartialEq)]
enum DjangoFieldType {
    Valid(&'static str),
    Relation(&'static str),
    None(String),
    ManyToMany,
}

#[derive(Debug, PartialEq)]
struct Fields {
    name: String,
    f_type: String,
    is_nullable: bool,
    default_value: Option<String>,
    max_length: Option<String>,
    tokens: Vec<Tok>,
}

impl Fields {
    fn new(
        name: String,
        f_type: String,
        is_nullable: bool,
        default_value: Option<String>,
        max_length: Option<String>,
        tokens: Vec<Tok>,
    ) -> Self {
        Self {
            name,
            f_type,
            is_nullable,
            default_value,
            max_length,
            tokens,
        }
    }
}

fn generate_struct_from_python(
    app_name: &str,
    struct_name: &str,
    python_code: &str,
    crate_requirements: &mut CrateRequirements,
) -> Result<(String, String, String)> {
    // トークンを収集
    let tokens = lex(python_code, Mode::Module)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| SyncDbError::TokenizationError(format!("{:?}", e)))?;

    let mut fields_vec: Vec<Fields> = Vec::new();

    let mut in_class_def = false;
    let mut current_class_name = String::new();
    let mut tokens_relational: Vec<Tok> = Vec::new();

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
                                            let mut max_length: Option<String> = None;

                                            // リレーショナルフィールド用トークン収集を開始
                                            if matches!(
                                                field_type.as_str(),
                                                "ForeignKey" | "ManyToManyField" | "OneToOneField"
                                            ) {
                                                tokens_relational.clear(); // 前回のトークンをクリア

                                                // フィールド定義の中身を収集
                                                let mut j = i + 5;
                                                while j < tokens.len() {
                                                    match &tokens[j].0 {
                                                        Tok::Newline => break, // 定義終了
                                                        _ => {
                                                            // トークンを収集
                                                            tokens_relational
                                                                .push(tokens[j].0.clone());
                                                        }
                                                    }
                                                    j += 1;
                                                }
                                            }

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
                                            fields_vec.push(Fields::new(
                                                name.clone(),
                                                field_type.clone(),
                                                is_nullable,
                                                default_value,
                                                max_length,
                                                tokens_relational.clone(),
                                            ));

                                            tokens_relational.clear();
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
        "#[allow(dead_code)]\n#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]\npub struct {struct_name}Db {{\n"
    );

    let mut create_struct = format!(
        "#[allow(dead_code)]\n#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]\npub struct Create{struct_name} {{\n"
    );

    rust_struct.push_str("    /// Primary Key\n    pub id: i64,\n");

    // CreateFooはidを持たない
    // create_struct.push_str("    /// Primary Key\n    pub id: i64,\n");

    let mut intermediate_structs: Vec<String> = Vec::new();

    for fields in &fields_vec {
        update_crate_requirements(crate_requirements, fields.f_type.as_str());

        let rust_type = map_django_field_to_rust_type(fields.f_type.as_str());

        if let DjangoFieldType::Relation(_) = &rust_type {
            // リレーションの解析処理で関連モデル名を取得
            let related_model = analyze_relation_field(fields.tokens.clone(), fields.name.as_str());

            if let Some(model) = related_model {
                rust_struct.push_str(&format!("\n    /// Related field to model: {}", model));
                create_struct.push_str(&format!("\n    /// Related field to model: {}", model));
            } else {
                rust_struct.push_str(&format!(
                    "\n    /// Related field: {} (unknown related model)",
                    fields.name
                ));
                create_struct.push_str(&format!(
                    "\n    /// Related field: {} (unknown related model)",
                    fields.name
                ));
            }
        }

        match rust_type {
            DjangoFieldType::Valid(ty) => {
                // コメント生成（default値とmax_lengthを含める）
                if let Some(comment) = generate_field_comment(fields) {
                    rust_struct.push_str(&format!("    {}\n", comment));
                    create_struct.push_str(&format!("    {}\n", comment));
                }

                let ty = if fields.is_nullable {
                    format!("Option<{}>", ty)
                } else {
                    ty.to_string()
                };
                rust_struct.push_str(&format!("    pub {}: {},\n", fields.name, ty));
                if fields.name != "id" {
                    create_struct.push_str(&format!("    pub {}: {},\n", fields.name, ty));
                }
            }
            DjangoFieldType::Relation(ty) => {
                rust_struct.push_str("    /// Note: Check on_delete behavior.\n");
                rust_struct.push_str(&format!("    pub {}: {},\n", fields.name, ty));
            }
            DjangoFieldType::ManyToMany => {
                // let sub_model_name =
                //     analyze_relation_field(fields.tokens.clone(), fields.name.as_str())
                //         .expect("to attribute or model name not found on ManyToManyField");

                let class_name = format!(
                    "{}{}{}Rel",
                    first_upper(app_name),
                    first_upper(struct_name),
                    // first_upper(&sub_model_name) // フィールド名でありモデル名ではない
                    first_upper(&fields.name)
                );

                // 中間テーブルの構造体を生成
                let intermediate_struct = format!(
                    "#[allow(dead_code)]\n#[derive(sqlx::FromRow, Debug, Clone)]\npub struct {} {{\n    /// Primary Key\n    pub id: i64,\n    pub {}_id: i64,\n    pub {}_id: i64,\n}}\n",
                    class_name,
                    struct_name.to_lowercase(),
                    &fields.name.to_lowercase()
                );
                intermediate_structs.push(intermediate_struct);
            }
            DjangoFieldType::None(ty) => {
                rust_struct.push_str(&format!("    /// No field type matches: {}\n", ty));
            }
        }
    }

    rust_struct.push_str("}\n");
    create_struct.push_str("}\n");
    Ok((rust_struct, create_struct, intermediate_structs.join("\n")))
}

/// Django model definition to be processed
struct ModelDefinition {
    app_name: &'static str,
    struct_name: &'static str,
    file_path: &'static str,
}

impl ModelDefinition {
    const fn new(
        app_name: &'static str,
        struct_name: &'static str,
        file_path: &'static str,
    ) -> Self {
        Self {
            app_name,
            struct_name,
            file_path,
        }
    }
}

fn map_django_field_to_rust_type(field_type: &str) -> DjangoFieldType {
    match field_type {
        "AutoField" => DjangoFieldType::Valid("u32"),
        "BigAutoField" => DjangoFieldType::Valid("u64"),
        "BigIntegerField" => DjangoFieldType::Valid("i64"),
        "BinaryField" => DjangoFieldType::Valid("Vec<u8>"),
        "BooleanField" => DjangoFieldType::Valid("bool"),
        "CharField" => DjangoFieldType::Valid("String"),
        "DateField" => DjangoFieldType::Valid("NaiveDate"),
        "DateTimeField" => DjangoFieldType::Valid("DateTime<Utc>"),
        "DecimalField" => DjangoFieldType::Valid("rust_decimal::Decimal"),
        "DurationField" => DjangoFieldType::Valid("chrono::Duration"),
        "EmailField" => DjangoFieldType::Valid("String"),
        "FileField" => DjangoFieldType::Valid("String"), // filepath
        "FilePathField" => DjangoFieldType::Valid("String"),
        "FloatField" => DjangoFieldType::Valid("f64"),
        "GeneratedField" => DjangoFieldType::Valid("String"), // フォールバック
        "GenericIPAddressField" => DjangoFieldType::Valid("std::net::IpAddr"),
        "ImageField" => DjangoFieldType::Valid("String"), // FileFieldと同じ扱い
        "IntegerField" => DjangoFieldType::Valid("i32"),
        "JSONField" => DjangoFieldType::Valid("Value"),
        "PositiveBigIntegerField" => DjangoFieldType::Valid("u64"), // 0 to 9223372036854775807
        "PositiveIntegerField" => DjangoFieldType::Valid("u32"),    // 0 to 2147483647
        "PositiveSmallIntegerField" => DjangoFieldType::Valid("u16"), // 0 to 32767
        "SlugField" => DjangoFieldType::Valid("String"),            // ascii only?
        "SmallAutoField" => DjangoFieldType::Valid("u16"),          //  1 to 32767
        "SmallIntegerField" => DjangoFieldType::Valid("i16"),       // -32768 to 32767
        "TextField" => DjangoFieldType::Valid("String"),
        "TimeField" => DjangoFieldType::Valid("chrono::NaiveTime"),
        "URLField" => DjangoFieldType::Valid("String"),
        "UUIDField" => DjangoFieldType::Valid("String"),

        // relationships
        "ForeignKey" | "OneToOneField" => DjangoFieldType::Relation("i64"),
        "ManyToManyField" => DjangoFieldType::ManyToMany,

        // unknown type
        unknown => DjangoFieldType::None(unknown.to_string()),
    }
}

fn generate_field_comment(fields: &Fields) -> Option<String> {
    let mut parts = Vec::new();

    if let Some(default) = &fields.default_value {
        parts.push(format!("Default: {}", default));
    }

    if let Some(length) = &fields.max_length {
        parts.push(format!("Max length: {}", length));
    }

    if parts.is_empty() {
        None
    } else {
        Some(format!("/// {}", parts.join(", ")))
    }
}

fn first_upper(s: &str) -> Cow<str> {
    let mut chars = s.chars();
    match chars.next() {
        None => Cow::Borrowed(""),
        Some(f) if f.is_uppercase() => Cow::Borrowed(s),
        Some(f) => Cow::Owned(f.to_uppercase().collect::<String>() + chars.as_str()),
    }
}

fn analyze_relation_field(
    tokens: Vec<Tok>, // Djangoのモデル定義のトークン
    _field_name: &str,
) -> Option<String> {
    let mut seen_to_keyword = false;
    let mut related_model = None; // to属性やモデル名を保持する変数
    let mut in_parentheses = false;

    for token in tokens {
        match token {
            Tok::Lpar => {
                in_parentheses = true;
            }
            Tok::Name { name } => {
                // `to` キーワード引数を検知
                if seen_to_keyword {
                    related_model = Some(name.clone());
                    break;
                }

                // 名前がリレーション引数として指定されていれば検知
                if name == "to" {
                    seen_to_keyword = true; // 次のトークンを関連モデル名とみなす
                } else if in_parentheses && related_model.is_none() {
                    // ForeignKey(Product) のような直接参照の場合
                    related_model = Some(name.clone());
                }
            }
            Tok::String { value, .. } => {
                // 文字列で指定されている場合はそのモデルとして解釈
                if seen_to_keyword || related_model.is_none() {
                    related_model = Some(value.clone());
                    break;
                }
            }
            _ => {}
        }
    }

    related_model
}

fn update_crate_requirements(crate_requirements: &mut CrateRequirements, field_type: &str) {
    match field_type {
        "BinaryField" => crate_requirements.use_serde_json = true,
        "DecimalField" => crate_requirements.use_rust_decimal = true,
        "JSONField" => crate_requirements.use_serde_json = true,
        "GenericIPAddressField" => crate_requirements.use_std_net = true,
        "DurationField" => {
            crate_requirements.use_chrono = true;
            crate_requirements.use_chrono_duration = true;
        }
        "DateField" => {
            crate_requirements.use_chrono = true;
            crate_requirements.use_chrono_naive_date = true;
        }
        "DateTimeField" => {
            crate_requirements.use_chrono = true;
            crate_requirements.use_chrono_datetimetz = true;
        }
        "TimeField" => {
            crate_requirements.use_chrono = true;
            crate_requirements.use_chrono_naive_time = true;
        }
        _ => {}
    }
}

struct CrateRequirements {
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
impl CrateRequirements {
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

    fn write_use_statements(&self, file: &mut fs::File) -> std::io::Result<()> {
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
            writeln!(file, "use chrono::{{{}}};", modules.join(", "))?;
        }
        if self.use_serde {
            writeln!(file, "use serde::{{Serialize, Deserialize}};")?;
        }
        if self.use_serde_json {
            writeln!(file, "use serde_json::Value;")?;
        }
        if self.use_rust_decimal {
            writeln!(file, "use rust_decimal::Decimal;")?;
        }
        if self.use_std_net {
            writeln!(file, "use std::net::IpAddr;")?;
        }
        if self.use_rust_decimal_macros {
            writeln!(file, "use rust_decimal_macros;")?;
        }
        if self.use_rust_decimal_ops {
            writeln!(file, "use rust_decimal_ops;")?;
        }

        Ok(())
    }
}
fn get_output_dir() -> Result<PathBuf> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").context("Failed to get CARGO_MANIFEST_DIR")?;

    Ok(PathBuf::from(manifest_dir)
        .parent() // マニフェストディレクトリの親に移動
        .ok_or_else(|| SyncDbError::PathError("Failed to get parent directory".to_string()))?
        .join("shared") // webappディレクトリに移動
        .join("models") // webappディレクトリに移動
        .join("src")
        .join("gen"))
}

fn main() -> Result<()> {
    let out_dir = get_output_dir()?;
    println!("Output directory: {}", out_dir.display());

    fs::create_dir_all(&out_dir).context("Output directory creation failed")?;

    let models = [
        ModelDefinition::new("wix", "Card", "./table_definition/wix/models.py"),
        ModelDefinition::new("wix", "CardType", "./table_definition/wix/models.py"),
        ModelDefinition::new("wix", "Product", "./table_definition/wix/models.py"),
        ModelDefinition::new("wix", "Klass", "./table_definition/wix/models.py"),
        ModelDefinition::new("wix", "RawCard", "./table_definition/wix/models.py"),
    ];

    let dest_path = Path::new(&out_dir).join("django_models.rs");

    println!("Output file: {}", dest_path.display());

    if dest_path.exists() {
        fs::remove_file(&dest_path).expect("Failed to remove existing django_models.rs");
    }

    let mut file = OpenOptions::new()
        .create(true) // ファイルがなければ作成
        .append(true) // 追記モード
        .open(&dest_path)
        .expect("Failed to open or create output file");

    let mut crate_req = CrateRequirements::new();
    crate_req.use_serde = true;

    let mut source_hash: HashMap<&str, String> = HashMap::new();

    // モデル定義を収集する
    let mut struct_defs = Vec::new();
    for model in models.iter() {
        let python_code = if let Some(code) = source_hash.get(model.file_path) {
            code
        } else {
            let code = fs::read_to_string(model.file_path)
                .with_context(|| format!("Failed to read file {}", model.file_path))?;
            source_hash.insert(model.file_path, code);
            source_hash.get(model.file_path).unwrap()
        };

        // 構造体生成コードと依存クレート解析
        let result = generate_struct_from_python(
            model.app_name,
            model.struct_name,
            python_code,
            &mut crate_req,
        )?;
        struct_defs.push(result);
    }

    // 必要なuse文を冒頭に書き込み
    crate_req
        .write_use_statements(&mut file)
        .context("Failed to write use statements to file")?;

    // 収集したモデル情報
    file.write_all(
        struct_defs
            .into_iter()
            .map(|def| format!("{}\n{}\n{}", def.0, def.1, def.2))
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )
    .context("Failed to write struct definitions to file")?;

    println!("Django model definitions successfully synced to Rust structs!");
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
