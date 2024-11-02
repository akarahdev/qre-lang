use crate::frontend::lexer::tokens::{Token, TokenType};
use std::io::Read;
use std::marker::PhantomData;

pub struct TokenIterator {
    pub(crate) vector: Vec<Token>,
    pub(crate) index: usize,
}

impl TokenIterator {
    pub fn next(&mut self) -> Option<&Token> {
        self.index += 1;
        self.vector.get(self.index - 1).clone()
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.vector.get(self.index).clone()
    }

    pub fn skip_newline(&mut self) {
        while let Some(tok) = self.peek().clone()
            && let TokenType::NewLine = tok.token_type
        {
            self.next();
        }
    }
}
