use crate::frontend::lexer::tokens::TokenType;
use crate::frontend::parser::ast::{AstCodeBlock, AstStatement};
use crate::frontend::parser::core::Parser;
use crate::frontend::span::Span;
use crate::match_token_type;

impl Parser {
    pub fn parse_code_block(&mut self) -> Option<AstCodeBlock> {
        match_token_type!(in self, let open_brace_tok: TokenType::OpenBrace => TokenType::OpenBrace);

        let mut stmts = Vec::new();
        loop {
            if let Some(peeked) = self.tokens.peek().cloned() {
                if peeked.token_type == TokenType::CloseBrace {
                    match_token_type!(in self, let close_brace_tok: TokenType::CloseBrace => TokenType::CloseBrace);
                    return Some(AstCodeBlock { statements: stmts });
                }
                let stmt = self.parse_statement();
                match stmt {
                    Ok(ok) => {
                        stmts.push(ok);
                    }
                    Err(err) => {
                        while let Some(peeked) = self.tokens.peek().cloned()
                            && (peeked.token_type != TokenType::Semicolon)
                        {
                            self.tokens.next();
                        }
                        self.errors.push(err);
                    }
                };
                match_token_type!(in self, let semi_tok: TokenType::Semicolon => TokenType::Semicolon);
                println!("stmts: {:#?}", stmts);
            } else {
                match_token_type!(in self, let close_brace_tok: TokenType::CloseBrace => TokenType::CloseBrace);
                return None;
            }
        }
    }

    pub fn parse_statement(&mut self) -> Result<AstStatement, (String, Span)> {
        let Some(tok) = self.tokens.peek().cloned() else {
            return Err((
                "expected valid statement, found EOF".to_string(),
                self.tokens.vector.last().cloned().unwrap().span,
            ));
        };
        match tok.token_type {
            TokenType::LoopKeyword => Err(("loops are not implemented yet".to_string(), tok.span)),
            TokenType::IfKeyword => Err((
                "if statements are not implemented yet".to_string(),
                tok.span,
            )),
            _ => Ok(AstStatement::Expression(self.parse_expression()?)),
        }
    }

}