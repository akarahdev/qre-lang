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

    DeclareVariable {
        name: String,
        type_hint: AstType,
        value: AstExpression,
    },
    ModifyVariable {
        name: String,
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
    NumberLiteral(String),
    StringLiteral(String),
    VariableLiteral(String),
    ArrayLiteral(Vec<AstExpression>),
    StructureLiteral(AstType, Vec<(String, AstExpression)>),

    Add(Box<AstExpression>, Box<AstExpression>),
    Sub(Box<AstExpression>, Box<AstExpression>),
    Mul(Box<AstExpression>, Box<AstExpression>),
    Div(Box<AstExpression>, Box<AstExpression>),
    Mod(Box<AstExpression>, Box<AstExpression>),

    Invoke {
        receiver: Box<AstExpression>,
        arguments: Vec<AstExpression>,
    },
}

enum AstType {
    Int32,
    Int64,
    Float32,
    Float64,
    ArrayOf(Box<AstType>),
    Structure(String),
    Inferred,
}
