use crate::frontend::lexer::iter::TokenIterator;
use crate::frontend::lexer::tokens::{Token, TokenType};
use crate::frontend::parser::ast::AstHeader::{Function, Import};
use crate::frontend::parser::ast::{AstCodeBlock, AstExpression, AstHeader, AstType, PathData};
use crate::frontend::span::Span;
use std::cell::OnceCell;
use std::cmp::PartialEq;
use std::fmt::format;
use std::iter::Peekable;
use std::ops::Add;

#[macro_use]
macro_rules! match_token_type {
    (in $self:expr, let $name:ident: $ty:expr => $token_type:pat) => {
        $self.tokens.skip_newline();
        let Some($name) = $self.tokens.next().clone() else {
            $self.errors.push((
                "expected OpenParen, found EOF".to_string(),
                $self.tokens.vector.last().unwrap().clone().span,
            ));
            return None;
        };
        let $token_type = $name.token_type else {
            $self.errors.push((
                format!("expected {:?}, found {:?}", $ty, $name.token_type),
                $self.tokens.vector.last().unwrap().clone().span,
            ));
            return None;
        };
    };
}

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

    pub fn parse_to_headers(&mut self) -> Vec<AstHeader> {
        let mut headers = Vec::new();
        while let Some(header) = self.parse_header() {
            headers.push(header);
        }
        headers
    }

    fn parse_header(&mut self) -> Option<AstHeader> {
        self.tokens.skip_newline();
        let Some(keyword_tok) = self.tokens.next() else {
            return None;
        };
        match keyword_tok.token_type {
            TokenType::ImportKeyword => match self.parse_identifier() {
                Ok(path) => Some(Import(path.name)),
                Err(err) => {
                    self.errors.push(err);
                    None
                }
            },
            TokenType::FnKeyword => {
                let ident = self.parse_identifier();
                let Ok(function_name) = ident else {
                    self.errors.push(ident.unwrap_err());
                    return None;
                };
                match_token_type!(in self, let open_paren_tok: TokenType::OpenParen => TokenType::OpenParen);
                match_token_type!(in self, let close_paren_tok: TokenType::CloseParen => TokenType::CloseParen);
                match_token_type!(in self, let arrow_tok: TokenType::Arrow => TokenType::Arrow);

                let Some(code_block) = self.parse_code_block() else {
                    return None;
                };
                Some(AstHeader::Function {
                    name: function_name,
                    parameters: vec![],
                    returns: AstType::Int32,
                    code_block,
                })
            }
            TokenType::StructKeyword => {
                self.errors.push((
                    "structs are currently not implemented".to_string(),
                    keyword_tok.span.clone(),
                ));
                return None;
            }
            _ => {
                self.errors.push((
                    format!(
                        "expected FnKeyword or StructKeyword, found {:?}",
                        keyword_tok.token_type
                    ),
                    keyword_tok.span.clone(),
                ));
                return None;
            }
        }
    }

    fn parse_code_block(&mut self) -> Option<AstCodeBlock> {
        match_token_type!(in self, let open_brace_tok: TokenType::CloseBrace => TokenType::CloseBrace);
        
        match_token_type!(in self, let close_brace_tok: TokenType::CloseBrace => TokenType::CloseBrace);

        Some(AstCodeBlock { statements: vec![] })
    }

    fn parse_identifier(&mut self) -> Result<PathData, (String, Span)> {
        let mut final_identifier = String::new();
        let mut tokens = Vec::new();

        loop {
            let Some(namespace_token) = self.tokens.next() else {
                return Err((
                    "expected Identifier, found EOF".to_string(),
                    self.tokens
                        .vector
                        .last()
                        .expect("last token available in this context")
                        .clone()
                        .span,
                ));
            };

            match &namespace_token.token_type {
                TokenType::Identifier { content } => {
                    final_identifier.push_str(&content);
                }
                _ => {}
            }

            tokens.push(namespace_token.clone());

            let Some(possibly_double_colon) = self.tokens.peek() else {
                return Ok(PathData { name: final_identifier, tokens });
            };
            let TokenType::DoubleColon = possibly_double_colon.token_type else {
                return Ok(PathData { name: final_identifier, tokens });
            };
            self.tokens.next();
            final_identifier.push_str("::");
        }
    }
}
