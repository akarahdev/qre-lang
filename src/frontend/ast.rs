use std::cell::OnceCell;

enum AstHeader {
    Function {
        name: String,
        parameters: Vec<(AstType, String)>,
        returns: AstType,
        code_block: AstCodeBlock,
    },
}

struct AstCodeBlock {
    statements: Vec<AstStatement>,
}

enum AstStatement {
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

enum AstExpression {
    NumberLiteral(String, OnceCell<AstType>),
    StringLiteral(String, OnceCell<AstType>),
    VariableLiteral(String, OnceCell<AstType>),
    ArrayLiteral(Vec<AstExpression>, OnceCell<AstType>),
    StructureLiteral(AstType, Vec<(String, AstExpression)>),

    Add(OnceCell<AstType>, Box<AstExpression>, Box<AstExpression>),
    Sub(OnceCell<AstType>,Box<AstExpression>, Box<AstExpression>),
    Mul(OnceCell<AstType>,Box<AstExpression>, Box<AstExpression>),
    Div(OnceCell<AstType>,Box<AstExpression>, Box<AstExpression>),
    Mod(OnceCell<AstType>,Box<AstExpression>, Box<AstExpression>),

    Invoke {
        receiver: Box<AstExpression>,
        arguments: Vec<AstExpression>,
        return_type: OnceCell<AstType>
    },
}

enum AstType {
    Int32,
    Int64,
    Float32,
    Float64,
    ArrayOf(Box<AstType>),
    Structure(String)
}
