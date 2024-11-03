use crate::frontend::lexer::iter::TokenIterator;
use crate::frontend::lexer::tokens::{Token, TokenType};
use crate::frontend::parser::ast::AstHeader::{Function, Import};
use crate::frontend::parser::ast::{
    AstCodeBlock, AstExpression, AstHeader, AstStatement, AstType, PathData,
};
use crate::frontend::span::Span;
use std::cell::OnceCell;
use std::cmp::PartialEq;
use std::fmt::format;
use std::iter::Peekable;
use std::ops::Add;

#[macro_use]
macro_rules! match_token_type {
    (in $self:expr, let $name:ident: $ty:expr => $token_type:pat) => {
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

                let return_type = match self.parse_type() {
                    Ok(t) => t,
                    Err(err) => {
                        self.errors.push(err);
                        AstType::Invalid
                    }
                };

                let Some(code_block) = self.parse_code_block() else {
                    return None;
                };
                Some(AstHeader::Function {
                    name: function_name,
                    parameters: vec![],
                    returns: return_type,
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

    fn parse_statement(&mut self) -> Result<AstStatement, (String, Span)> {
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

    fn parse_expression(&mut self) -> Result<AstExpression, (String, Span)> {
        self.parse_factor()
    }

    fn parse_factor(&mut self) -> Result<AstExpression, (String, Span)> {
        let mut expr = self.parse_term();
        while let Some(tok) = self.tokens.peek().cloned() {
            match tok.token_type {
                TokenType::Star => {
                    self.tokens.next();
                    let rhs = self.parse_factor()?;
                    expr = expr.map(|lhs| AstExpression::Mul {
                        ty: OnceCell::new(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op_tok: tok.clone(),
                    });
                }
                TokenType::Slash => {
                    self.tokens.next();
                    let rhs = self.parse_factor()?;
                    expr = expr.map(|lhs| AstExpression::Div {
                        ty: OnceCell::new(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op_tok: tok.clone(),
                    });
                }
                _ => break,
            };
        }
        expr
    }

    fn parse_term(&mut self) -> Result<AstExpression, (String, Span)> {
        let mut expr = self.parse_base_value();
        while let Some(tok) = self.tokens.peek().cloned() {
            match tok.token_type {
                TokenType::Plus => {
                    self.tokens.next();
                    let rhs = self.parse_base_value()?;
                    expr = expr.map(|lhs| AstExpression::Add {
                        ty: OnceCell::new(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op_tok: tok.clone(),
                    });
                }
                TokenType::Minus => {
                    self.tokens.next();
                    let rhs = self.parse_base_value()?;
                    expr = expr.map(|lhs| AstExpression::Sub {
                        ty: OnceCell::new(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op_tok: tok.clone(),
                    });
                }
                _ => break,
            };
        }
        expr
    }

    fn parse_base_value(&mut self) -> Result<AstExpression, (String, Span)> {
        let Some(tok) = self.tokens.peek().cloned() else {
            return Err((
                "expected base value, found EOF".to_string(),
                self.tokens.vector.last().cloned().unwrap().span,
            ));
        };
        self.tokens.next();
        match tok.clone().token_type {
            TokenType::Number { content } => Ok(AstExpression::NumberLiteral {
                content,
                ty: OnceCell::new(),
                token: tok.clone(),
            }),
            _ => Err((
                format!("expected base value, found {:?}", tok.clone().token_type),
                self.tokens.vector.last().cloned().unwrap().span,
            )),
        }
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
                return Ok(PathData {
                    name: final_identifier,
                    tokens,
                });
            };
            let TokenType::DoubleColon = possibly_double_colon.token_type else {
                return Ok(PathData {
                    name: final_identifier,
                    tokens,
                });
            };
            self.tokens.next();
            final_identifier.push_str("::");
        }
    }

    fn parse_type(&mut self) -> Result<AstType, (String, Span)> {
        let identifier = self.parse_identifier()?;
        match identifier.name.as_str() {
            "i32" => Ok(AstType::Int32),
            "i64" => Ok(AstType::Int64),
            "f32" => Ok(AstType::Float32),
            "f64" => Ok(AstType::Float64),
            "void" => Ok(AstType::Void),
            _ => Ok(AstType::Structure(identifier))
        }
    }
}
