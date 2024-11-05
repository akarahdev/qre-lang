use crate::frontend::lexer::tokens::Token;
use std::cell::OnceCell;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstHeader {
    Import(String),
    Function {
        name: PathData,
        parameters: Vec<(AstType, String)>,
        returns: AstType,
        code_block: AstCodeBlock,
    },
    Struct {
        name: PathData,
        fields: Vec<(String, AstType)>
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AstCodeBlock {
    pub statements: Vec<AstStatement>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstStatement {
    Comment(String),
    Expression(AstExpression),

    ModifyVariable {
        name: String,
        ty: OnceCell<AstType>,
        value: AstExpression,
    },
    IfStatement {
        cond: AstExpression,
        if_true: AstCodeBlock,
        if_false: AstCodeBlock,
    },
    WhileStatement {
        cond: AstExpression,
        do_true: AstCodeBlock,
    },
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PathData {
    pub(crate) name: String,
    pub(crate) token: Token,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstExpression {
    NumberLiteral {
        content: String,
        ty: OnceCell<AstType>,
        token: Token,
    },
    StringLiteral {
        content: String,
        ty: OnceCell<AstType>,
        token: Token,
    },
    VariableLiteral {
        content: String,
        ty: OnceCell<AstType>,
        token: Token,
    },
    PathLiteral(PathData),
    ArrayLiteral {
        content: Vec<AstExpression>,
        ty: OnceCell<AstType>,
        open_bracket_tok: Token,
        close_bracket_tok: Token,
    },
    StructureLiteral {
        ty: AstType,
        fields: Vec<(String, AstExpression)>,
    },
    TypeLiteral {
        ty: AstType,
        token: Token,
    },

    Add {
        ty: OnceCell<AstType>,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
        op_tok: Token,
    },
    Sub {
        ty: OnceCell<AstType>,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
        op_tok: Token,
    },
    Mul {
        ty: OnceCell<AstType>,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
        op_tok: Token,
    },
    Div {
        ty: OnceCell<AstType>,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
        op_tok: Token,
    },
    Mod {
        ty: OnceCell<AstType>,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
        op_tok: Token,
    },

    Invoke {
        receiver: Box<AstExpression>,
        arguments: Vec<AstExpression>,
        return_type: OnceCell<AstType>,
        open_paren_span: Token,
        close_paren_tok: Token,
        resolve_as_ufcs: bool
    },

    Index {
        ty: OnceCell<AstType>,
        base: Box<AstExpression>,
        other: Box<AstExpression>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstType {
    Int32,
    Int64,
    Float32,
    Float64,
    ArrayOf(Box<AstType>),
    Structure(PathData),
    Void,
    Invalid,
    
    UnionOf(Box<AstType>, Box<AstType>)
}
