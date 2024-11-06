#![feature(let_chains)]
#![allow(dead_code)]

use crate::frontend::lexer::iter::TokenIterator;
use crate::frontend::lexer::structs::Lexer;
use crate::frontend::typecheck::data::TypeInformation;
use frontend::parser::core::Parser;
use std::alloc::System;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;
use crate::frontend::parser::ast::AstHeader;

mod backend;
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

    let Ok(mut headers) = ast else {
        println!("Errs: {:#?}", ast.unwrap_err());
        exit(1);
    };

    let mut type_info = TypeInformation {
        names: HashMap::new(),
    };
    headers
        .iter()
        .for_each(|x| x.gather_type_information(&mut type_info));

    for header in &mut headers {
        if let AstHeader::Function { ref mut code_block, ref mut locals, .. } = header {
            code_block.annotate_type_information(
                &mut type_info,
                locals
            );
        }
    }

    println!("Types: {:#?}", type_info);
    println!("Annotated: {:#?}", headers);
}
