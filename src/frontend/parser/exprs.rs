use std::cell::OnceCell;
use crate::frontend::lexer::tokens::TokenType;
use crate::frontend::parser::ast::AstExpression;
use crate::frontend::parser::core::Parser;
use crate::frontend::span::Span;

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> Result<AstExpression, (String, Span)> {
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
}