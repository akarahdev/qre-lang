use crate::frontend::lexer::tokens::TokenType;
use crate::frontend::parser::ast::AstExpression;
use crate::frontend::parser::core::Parser;
use crate::frontend::span::Span;
use std::cell::OnceCell;
use crate::match_token_type;

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> Result<AstExpression, (String, Span)> {
        self.parse_ufcs()
    }

    fn parse_ufcs(&mut self) -> Result<AstExpression, (String, Span)> {
        let mut expr = self.parse_factor()?;
        while let Some(tok) = self.tokens.peek().cloned()
            && tok.token_type == TokenType::Dot {
            self.tokens.next();
            let rhs = self.parse_factor()?;

            match rhs {
                AstExpression::Invoke { receiver, arguments, open_paren_span, close_paren_tok, return_type } => {
                    let mut tmp_args = Vec::with_capacity(arguments.len() + 1);
                    tmp_args.push(expr);
                    tmp_args.extend(arguments);
                    

                    expr = AstExpression::Invoke {
                        receiver,
                        arguments: tmp_args,
                        open_paren_span,
                        close_paren_tok,
                        return_type,
                        resolve_as_ufcs: true
                    }
                }
                _ => self.errors.push((
                    "UFCS must be followed by a function invocation".to_string(),
                    tok.span.clone()
                )),
            }
        }
        Ok(expr)
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
        let mut expr = self.parse_postfix_exprs();
        while let Some(tok) = self.tokens.peek().cloned() {
            match tok.token_type {
                TokenType::Plus => {
                    self.tokens.next();
                    let rhs = self.parse_postfix_exprs()?;
                    expr = expr.map(|lhs| AstExpression::Add {
                        ty: OnceCell::new(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op_tok: tok.clone(),
                    });
                }
                TokenType::Minus => {
                    self.tokens.next();
                    let rhs = self.parse_postfix_exprs()?;
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

    fn parse_postfix_exprs(&mut self) -> Result<AstExpression, (String, Span)> {
        let mut expr = self.parse_base_value()?;
        while let Some(tok) = self.tokens.peek().cloned() {
            match tok.token_type {
                TokenType::OpenBracket => {
                    self.tokens.next();

                    let index_by = self.parse_expression()?;

                    let Some(close_brack_tok) = self.tokens.peek().cloned() else {
                        return Err((
                            "expected CloseBracket, found EOF".to_string(),
                            self.tokens.vector.last().cloned().unwrap().span,
                        ));
                    };
                    let TokenType::CloseBracket = &close_brack_tok.token_type else {
                        return Err((
                            format!("expected CloseBracket, found {:?}", close_brack_tok.token_type),
                            close_brack_tok.span,
                        ));
                    };
                    self.tokens.next();

                    expr = AstExpression::Index {
                        ty: OnceCell::new(),
                        base: Box::new(expr),
                        other: Box::new(index_by),
                    }
                }
                TokenType::OpenParen => {
                    self.tokens.next();

                    let mut arguments = Vec::new();
                    loop {
                        if let Some(peeked) = self.tokens.peek().cloned()
                            && peeked.token_type == TokenType::CloseParen {
                            break;
                        };
                        
                        let arg = self.parse_expression()?;
                        arguments.push(arg);

                        if let Some(peeked) = self.tokens.peek().cloned()
                            && peeked.token_type != TokenType::Comma {
                            break;
                        };
                        self.tokens.next();
                    }

                    let Some(close_paren_tok) = self.tokens.peek().cloned() else {
                        return Err((
                            "expected CloseParen, found EOF".to_string(),
                            self.tokens.vector.last().cloned().unwrap().span,
                        ));
                    };
                    let TokenType::CloseParen = &close_paren_tok.token_type else {
                        return Err((
                            format!("expected CloseParen, found {:?}", close_paren_tok.token_type),
                            close_paren_tok.span,
                        ));
                    };
                    self.tokens.next();

                    expr = AstExpression::Invoke {
                        receiver: Box::new(expr),
                        arguments,
                        return_type: OnceCell::new(),
                        open_paren_span: tok,
                        close_paren_tok,
                        resolve_as_ufcs: false
                    }
                }
                _ => break
            };
        }
        Ok(expr)
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
            TokenType::Identifier { content} => Ok(AstExpression::VariableLiteral {
                content,
                ty: OnceCell::new(),
                token: tok,
            }),
            _ => Err((
                format!("expected base value, found {:?}", tok.clone().token_type),
                self.tokens.vector.last().cloned().unwrap().span,
            )),
        }
    }
}
