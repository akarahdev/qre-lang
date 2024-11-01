#![allow(dead_code)]

use crate::frontend::lexer::iter::TokenIterator;
use crate::frontend::lexer::structs::Lexer;
use crate::frontend::parser::structs::Parser;
use std::fs::read_to_string;

mod frontend;

fn main() {
    let entries = std::fs::read_dir("./src/").unwrap();

    let mut handles = vec![];
    for entry in entries {
        handles.push(std::thread::spawn(move || {
            let path = entry.unwrap().path().to_str().unwrap().to_string();
            let file = read_to_string(&path).unwrap();
            let mut lexer = Lexer::new(path, file);
            let _ = lexer.lex();
            lexer.tokens
        }));
    }

    let tokens = handles
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .flatten()
        .collect::<Vec<_>>();

    println!("Results: {:#?}", &tokens);

    let mut parser = Parser {
        tokens: TokenIterator {
            vector: tokens,
            index: 0usize,
        },
        errors: vec![],
    };
    let ast = parser.parse();

    println!("Parsing: {:#?}", ast);
}
