use std::cell::OnceCell;
use crate::frontend::parser::ast::AstHeader;
use crate::frontend::lexer::tokens::Token;

pub struct Parser<'a> {
    pub tokens: &'a dyn Iterator<Item = &'a Token>,
}

impl<'a> Parser<'a> {
    pub fn parse_to_headers(&mut self) -> Vec<AstHeader> {
        let mut headers = Vec::new();

        headers
    }
}