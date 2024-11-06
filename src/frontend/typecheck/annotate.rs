use crate::frontend::parser::ast::{AstCodeBlock, AstExpression, AstHeader, AstStatement, AstType};
use crate::frontend::typecheck::data::TypeInformation;

impl AstStatement {
    pub fn annotate_type_information(&mut self, type_information: &TypeInformation, function: &mut AstHeader) {
        match self {
            AstStatement::Comment(content) => {

            }
            AstStatement::Expression(expr) => {
                expr.annotate_type_information(type_information, function);
            }
            AstStatement::ModifyVariable { name, ty, value } => {
                let AstHeader::Function { locals, .. } = function else {
                    panic!();
                };
                value.annotate_type_information(type_information, function);
                
                locals.insert(name.clone(), ty.get());
                
                
            }
            AstStatement::IfStatement { cond, if_true, if_false } => {}
            AstStatement::WhileStatement { cond, do_true } => {}
        }
    }
}

impl AstExpression {
    pub fn annotate_type_information(&mut self, type_data: &TypeInformation, function: &AstHeader) {
        match self {
            AstExpression::NumberLiteral { content, ty, .. } => {
                ty.set(AstType::Int32)
                    .expect("type must not be set");
            }
            AstExpression::StringLiteral { content, ty, .. } => {
                ty.set(AstType::Structure("std::string".to_string()))
                    .expect("type must not be set");
            }
            AstExpression::VariableLiteral { content, ty, .. } => {

            }
            AstExpression::PathLiteral(_) => {}
            AstExpression::ArrayLiteral { .. } => {}
            AstExpression::StructureLiteral { .. } => {}
            AstExpression::TypeLiteral { .. } => {}
            AstExpression::Add { .. } => {}
            AstExpression::Sub { .. } => {}
            AstExpression::Mul { .. } => {}
            AstExpression::Div { .. } => {}
            AstExpression::Mod { .. } => {}
            AstExpression::Invoke { .. } => {}
            AstExpression::Index { .. } => {}
        }
    }
}