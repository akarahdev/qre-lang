use crate::span::Span;

pub struct Token {
    token_type: TokenType,
    span: Span
}

pub enum TokenType {
    Identifier { content: String },
    StringValue { content: String },
    CStringValue { content: String },
    Comment { content: String },

    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    Comma,
    Colon,
    DoubleColon,
    Semicolon,
    Dot,
    Arrow,

    Tilde,
    Grave,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Exclamation,
    At,
    Hash,
    Dollar,
    Caret,
    Ampersand,
    QuestionMark,
    VerticalLine,
    Backslash,

    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    DoubleEqual
}