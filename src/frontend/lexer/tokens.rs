use crate::frontend::span::Span;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Identifier { content: String },
    StringValue { content: String },
    Comment { content: String },
    Number { content: String },

    ImportKeyword,
    FnKeyword,
    StructKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    LoopKeyword,
    BreakKeyword,
    InterfaceKeyword,
    ForEachKeyword,
    CKeyword,

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
    DoubleDot,
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
    DoubleEqual,
}
