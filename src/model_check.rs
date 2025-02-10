use rustpython_parser_core::mode::Mode;
use rustpython_parser::lexer::lex;
use rustpython_parser::Tok;

fn main() {
    let python_source = r#"
class Card(models.Model):
    name = models.CharField(max_length=256)
    created_at = models.DateTimeField(auto_now_add=True)
    "#;

    let tokens = lex(python_source, Mode::Module);

    let mut class_name = String::new();
    let mut fields = vec![];

    let mut tokens_iter = tokens.peekable();
    while let Some(Ok((tok, _))) = tokens_iter.next() {
        match tok {
            Tok::Class => {
                // 次のトークンがクラス名
                if let Some(Ok((Tok::Name { name }, _))) = tokens_iter.next() {
                    class_name = name;
                }
            }
            Tok::Name { name } => {
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
                                fields.push((field_name, field_type));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    println!("Class Name: {}", class_name);
    println!("Fields:");
    for (field_name, field_type) in fields {
        println!("  {}: {}", field_name, field_type);
    }
}