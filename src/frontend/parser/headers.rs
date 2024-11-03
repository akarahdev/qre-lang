use crate::frontend::lexer::tokens::TokenType;
use crate::frontend::parser::ast::AstHeader::Import;
use crate::frontend::parser::ast::{AstHeader, AstType};
use crate::frontend::parser::core::Parser;
use crate::match_token_type;

impl Parser {
    pub fn parse_to_headers(&mut self) -> Vec<AstHeader> {
        let mut headers = Vec::new();
        while let Some(header) = self.parse_header() {
            headers.push(header);
        }
        headers
    }

    pub fn parse_header(&mut self) -> Option<AstHeader> {
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
}
