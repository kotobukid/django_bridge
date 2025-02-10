use rustpython_parser::{Parse, ast};
use rustpython_parser::{lexer::lex, Mode};

fn main() {
    let python_source = "print('Hello world')";
    // let python_statements = ast::Suite::parse(python_source).unwrap();  // statements
    // let python_expr = ast::Expr::parse(python_source).unwrap();  // or expr

    let tokens: Vec<_> = lex(python_source, Mode::Module).collect();
    // assert!(tokens.all(|t| t.is_ok()));

    println!("{:?}", tokens);
}