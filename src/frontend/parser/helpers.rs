use crate::frontend::lexer::tokens::TokenType;
use crate::frontend::parser::ast::{AstType, PathData};
use crate::frontend::parser::core::Parser;
use crate::frontend::span::Span;

impl Parser {
    pub fn parse_identifier(&mut self) -> Result<PathData, (String, Span)> {
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

    pub fn parse_type(&mut self) -> Result<AstType, (String, Span)> {
        let identifier = self.parse_identifier()?;
        match identifier.name.as_str() {
            "i32" => Ok(AstType::Int32),
            "i64" => Ok(AstType::Int64),
            "f32" => Ok(AstType::Float32),
            "f64" => Ok(AstType::Float64),
            "void" => Ok(AstType::Void),
            _ => Ok(AstType::Structure(identifier)),
        }
    }
}
