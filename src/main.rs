use crate::lexer::Lexer;
use std::fs::read_to_string;

mod lexer;
mod span;
mod tokens;

fn main() {
    let file = read_to_string("./example/hello.qre").unwrap();

    let mut lexer = Lexer::new("./example/hello.qre".to_string(), file);
    let tokens = lexer.lex();

    println!("{:?}", tokens);
}
