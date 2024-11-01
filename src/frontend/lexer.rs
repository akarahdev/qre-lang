use crate::frontend::span::Span;
use crate::frontend::tokens::{Token, TokenType};

pub struct Lexer {
    file_name: String,
    file_contents: String,
    pub(crate) tokens: Vec<Token>,

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
                ch if ch == '"' || ch == '\'' => {
                    let mut content = String::new();
                    while self.peek_char() != ch {
                        if self.peek_char() == '\\' {
                            self.read_char();
                            match self.read_char() {
                                '\\' => {
                                    content.push('\\');
                                }
                                'n' => {
                                    content.push('\n');
                                }
                                't' => {
                                    content.push('\t');
                                }
                                '0' => {
                                    content.push('\0');
                                }
                                ch => panic!("unknown special character \\{}", ch),
                            }
                        }
                        content.push(self.read_char());
                    }
                    self.read_char();
                    self.push_token(TokenType::StringValue { content })
                }
                ch if ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) => {
                    let mut content = String::new();
                    content.push(ch);

                    while self.peek_char().is_alphanumeric() {
                        content.push(self.read_char());
                    }
                    if !content.is_empty() {
                        match content.to_lowercase().as_str() {
                            "import" => self.push_token(TokenType::ImportKeyword),
                            "break" => self.push_token(TokenType::BreakKeyword),
                            "else" => self.push_token(TokenType::ElseKeyword),
                            "fn" => self.push_token(TokenType::FnKeyword),
                            "if" => self.push_token(TokenType::IfKeyword),
                            "interface" => self.push_token(TokenType::InterfaceKeyword),
                            "loop" => self.push_token(TokenType::LoopKeyword),
                            "struct" => self.push_token(TokenType::StructKeyword),
                            "while" => self.push_token(TokenType::WhileKeyword),
                            "foreach" => self.push_token(TokenType::ForEachKeyword),
                            "c" => self.push_token(TokenType::CKeyword),
                            _ => self.push_token(TokenType::Identifier { content }),
                        }
                    }
                }
                ' ' | '\t' => {}
                ':' => {
                    if self.peek_char() == ':' {
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
                '=' => {
                    if self.peek_char() == '=' {
                        self.push_token(TokenType::DoubleEqual);
                    } else {
                        self.push_token(TokenType::Equal);
                    }
                }
                '>' => {
                    if self.peek_char() == '=' {
                        self.push_token(TokenType::GreaterThanOrEqual);
                    } else {
                        self.push_token(TokenType::GreaterThan);
                    }
                }
                '<' => {
                    if self.peek_char() == '=' {
                        self.push_token(TokenType::LessThanOrEqual);
                    } else {
                        self.push_token(TokenType::LessThan);
                    }
                }
                '(' => self.push_token(TokenType::OpenParen),
                ')' => self.push_token(TokenType::CloseParen),
                '[' => self.push_token(TokenType::OpenBracket),
                ']' => self.push_token(TokenType::CloseBracket),
                '{' => self.push_token(TokenType::OpenBrace),
                '}' => self.push_token(TokenType::CloseBrace),

                '+' => self.push_token(TokenType::Plus),
                '-' => {
                    if self.peek_char() == '>' {
                        self.push_token(TokenType::Arrow);
                    } else {
                        self.push_token(TokenType::Minus);
                    }
                }
                '*' => self.push_token(TokenType::Star),
                '/' => {
                    if self.peek_char() == '/' {
                        let mut content = String::new();
                        while self.peek_char() != '\n' {
                            content.push(self.read_char());
                        }
                        self.push_token(TokenType::Comment { content });
                    } else {
                        self.push_token(TokenType::Slash);
                    }
                }
                '%' => self.push_token(TokenType::Percent),
                '$' => self.push_token(TokenType::Dollar),
                '&' => self.push_token(TokenType::Ampersand),
                '^' => self.push_token(TokenType::Caret),
                '\\' => self.push_token(TokenType::Backslash),
                '`' => self.push_token(TokenType::Grave),
                '~' => self.push_token(TokenType::Tilde),
                ';' => self.push_token(TokenType::Semicolon),
                ',' => self.push_token(TokenType::Comma),

                '#' => self.push_token(TokenType::Hash),
                '@' => self.push_token(TokenType::At),
                '!' => {
                    if self.peek_char() == '=' {
                        self.push_token(TokenType::NotEqual)
                    } else {
                        self.push_token(TokenType::Exclamation)
                    }
                }
                '?' => self.push_token(TokenType::QuestionMark),
                '|' => self.push_token(TokenType::VerticalLine),

                '\r' => {}
                '\n' => self.push_token(TokenType::NewLine),

                ch => panic!("unknown character `{}`", ch),
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
