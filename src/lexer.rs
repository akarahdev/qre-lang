use crate::span::Span;
use crate::tokens::{Token, TokenType};
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::{Add, RangeBounds};
use std::str::Chars;

pub struct Lexer {
    file_name: String,
    file_contents: String,
    tokens: Vec<Token>,

    character_offset: usize,

    row: usize,
    column: usize,
}

impl Lexer {
    pub fn new(file_name: String, file_contents: String) -> Lexer {
        Lexer {
            file_name,
            file_contents,
            tokens: vec![],

            character_offset: 0,
            row: 0,
            column: 0,
        }
    }

    pub fn generate_span(&self) -> Span {
        Span {
            column_start: self.column,
            column_end: self.column,

            row_start: self.row,
            row_end: self.row,

            file_name: self.file_name.clone(),
        }
    }

    pub fn read_char(&mut self) -> char {
        let result = self
            .file_contents
            .chars()
            .nth(self.character_offset)
            .unwrap_or('\0');
        self.character_offset += 1;
        self.column += 1;
        if result == '\n' {
            self.column = 0;
            self.row += 1;
        }
        result
    }

    pub fn peek_char(&mut self) -> char {
        self.file_contents
            .chars()
            .nth(self.character_offset)
            .unwrap_or('\0')
    }

    pub fn lex(&mut self) -> &Vec<Token> {
        self.tokens.clear();

        'outer: loop {
            println!("tokens: {:?}", self.tokens);
            match self.read_char() {
                '\0' => break 'outer,
                ch if ('0'..='9').contains(&ch) => {
                    let mut content = String::new();
                    content.push(ch);
                    while ('0'..='9').contains(&self.peek_char()) || self.peek_char() == '.' {
                        content.push(self.read_char());
                    }
                    self.push_token(TokenType::Number { content })
                }
                ch if ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) => {
                    let mut content = String::new();
                    content.push(ch);

                    while self.peek_char().is_alphanumeric() {
                        content.push(self.read_char());
                        println!("content: {}", content);
                    }
                    if !content.is_empty() {
                        self.push_token(TokenType::Identifier { content })
                    }
                }
                ' ' | '\t' => {}
                ':' => {
                    if (self.peek_char() == ':') {
                        self.push_token(TokenType::DoubleColon);
                    } else {
                        self.push_token(TokenType::Colon);
                    }
                }
                '.' => {
                    if self.peek_char() == '.' {
                        self.push_token(TokenType::DoubleDot);
                    } else {
                        self.push_token(TokenType::Dot);
                    }
                }
                '(' => self.push_token(TokenType::OpenParen),
                ')' => self.push_token(TokenType::CloseParen),
                '[' => self.push_token(TokenType::OpenBracket),
                ']' => self.push_token(TokenType::CloseBracket),
                '{' => self.push_token(TokenType::OpenBrace),
                '}' => self.push_token(TokenType::CloseBrace),
                '\n' => self.push_token(TokenType::NewLine),

                ch => panic!("unknown character '{}'", ch),
            }
        }

        &self.tokens
    }

    pub fn push_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            span: self.generate_span(),
        })
    }
}
