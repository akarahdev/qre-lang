use crate::frontend::lexer::tokens::Token;

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
}
