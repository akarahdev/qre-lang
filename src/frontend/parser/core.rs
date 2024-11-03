use crate::frontend::lexer::iter::TokenIterator;
use crate::frontend::parser::ast::AstHeader;
use crate::frontend::span::Span;

pub struct Parser {
    pub tokens: TokenIterator,
    pub errors: Vec<(String, Span)>,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<AstHeader>, Vec<(String, Span)>> {
        let parsed = self.parse_to_headers();
        if self.errors.is_empty() {
            Result::Ok(parsed)
        } else {
            Result::Err(self.errors.clone())
        }
    }







}