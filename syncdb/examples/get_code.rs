use rustpython_parser::lexer::lex;
use rustpython_parser::Tok;
use rustpython_parser_core::mode::Mode;
use std::fs;

fn main() -> std::io::Result<()> {
    // 解析対象のPythonモジュール
    let file_path = "table_definition/wix/models.py"; // Pythonファイルのパス
    let target_class = "Card"; // 対象のクラス名

    // Pythonコードをファイルから読み取る
    let python_source = fs::read_to_string(file_path)?;

    // トークン解析
    let tokens = lex(&python_source, Mode::Module);

    let mut _class_name = String::new();
    let mut fields = vec![];
    let mut inside_target_class = false;

    let mut tokens_iter = tokens.peekable();
    while let Some(Ok((tok, _))) = tokens_iter.next() {
        match tok {
            Tok::Class => {
                // 次のトークンがクラス名
                if let Some(Ok((Tok::Name { name }, _))) = tokens_iter.next() {
                    _class_name = name;
                    inside_target_class = _class_name == target_class; // 対象クラスのみ処理
                }
            }
            Tok::Name { name } => {
                if inside_target_class {
                    // フィールド名の場合
                    if let Some(Ok((Tok::Equal, _))) = tokens_iter.peek() {
                        let field_name = name.clone();
                        tokens_iter.next(); // "=" を消費

                        // 次のトークンがフィールド型（例: models.CharField）
                        if let Some(Ok((Tok::Name { name: type_name }, _))) = tokens_iter.next() {
                            if type_name == "models" {
                                // `.`をスキップ
                                tokens_iter.next();
                                if let Some(Ok((Tok::Name { name: field_type }, _))) =
                                    tokens_iter.next()
                                {
                                    // アトリビュートを収集
                                    let mut attributes = vec![];
                                    if let Some(Ok((Tok::Lpar, _))) = tokens_iter.peek() {
                                        tokens_iter.next(); // "(" を消費

                                        while let Some(Ok((tok, _))) = tokens_iter.next() {
                                            if let Tok::Name {
                                                name: ref attr_name,
                                            } = tok
                                            {
                                                if let Some(Ok((Tok::Equal, _))) =
                                                    tokens_iter.next()
                                                {
                                                    if let Some(Ok((tok_value, _))) =
                                                        tokens_iter.next()
                                                    {
                                                        // 値をフォーマット
                                                        let formatted_value =
                                                            format_tok_value(&tok_value);
                                                        attributes.push((
                                                            attr_name.clone(),
                                                            formatted_value,
                                                        ));
                                                    }
                                                }
                                            }
                                            if let Tok::Rpar = tok {
                                                break;
                                            }
                                        }
                                    }

                                    fields.push((field_name, field_type, attributes));
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if inside_target_class {
        println!("Class Name: {target_class}");
        println!("Fields:");
        for (field_name, field_type, attributes) in fields {
            println!("  {field_name}: {field_type} {{");
            for (attr_name, attr_value) in attributes {
                println!("    {attr_name}: {attr_value}");
            }
            println!("  }}");
        }
    } else {
        println!("Class '{target_class}' not found in {file_path}");
    }

    Ok(())
}

/// トークン値を分かりやすい形式にフォーマット
fn format_tok_value(tok: &Tok) -> String {
    match tok {
        Tok::Int { value } => value.to_string(), // 整数を直接文字列化
        Tok::String { value, .. } => format!("{value:?}"), // 文字列をエスケープ付きで出力
        Tok::True => "true".to_string(),         // PythonのTrueをRustのtrueにマッピング
        Tok::False => "false".to_string(),       // PythonのFalseをRustのfalseにマッピング
        Tok::Name { name } => name.clone(),      // その他の名前をそのまま使用
        _ => format!("{tok:?}"),               // 未知の型（デバッグ用）
    }
}
