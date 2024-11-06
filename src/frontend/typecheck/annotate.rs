use std::cell::OnceCell;
use std::collections::HashMap;
use std::str::FromStr;
use crate::frontend::parser::ast::{AstCodeBlock, AstExpression, AstHeader, AstStatement, AstType};
use crate::frontend::typecheck::data::TypeInformation;

impl AstCodeBlock {
    pub fn annotate_type_information(
        &mut self,
        type_information: &TypeInformation,
        locals: &mut HashMap<String, AstType>
    ) {
        for stmt in &mut self.statements {
            stmt.annotate_type_information(type_information, locals);
        }
    }
}

impl AstStatement {
    pub fn annotate_type_information(
        &mut self,
        type_information: &TypeInformation,
        locals: &mut HashMap<String, AstType>
    ) {
        match self {
            AstStatement::Comment(content) => {}
            AstStatement::Expression(expr) => {
                expr.annotate_type_information(type_information, locals);
            }
            AstStatement::ModifyVariable { name, ty, value } => {
                let var_ty = value.annotate_type_information(type_information, locals);
                locals.insert(name.clone(), var_ty);
            }
            AstStatement::IfStatement {
                cond,
                if_true,
                if_false,
            } => {
                cond.annotate_type_information(type_information, locals);
            }
            AstStatement::WhileStatement { cond, do_true } => {}
        }
    }
}

impl AstExpression {
    pub fn get_type(
        &self
    ) -> AstType {
        match self {
            AstExpression::NumberLiteral { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::StringLiteral { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::VariableLiteral { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::PathLiteral(_) => AstType::Invalid,
            AstExpression::ArrayLiteral { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::StructureLiteral { ty, .. } => ty.clone(),
            AstExpression::TypeLiteral { ty, .. } => ty.clone(),
            AstExpression::Add { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::Sub { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::Mul { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::Div { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::Mod { ty, .. } => ty.get().cloned().unwrap(),
            AstExpression::Invoke { return_type, .. } => return_type.get().cloned().unwrap(),
            AstExpression::Index { ty, .. } => ty.get().cloned().unwrap(),
        }
    }

    pub fn annotate_binop(
        lhs: &mut AstExpression, rhs: &mut AstExpression, ty: &mut OnceCell<AstType>,
        type_data: &TypeInformation, locals: &HashMap<String, AstType>) -> AstType {
        lhs.annotate_type_information(type_data, locals);
        rhs.annotate_type_information(type_data, locals);
        if(lhs.get_type() == rhs.get_type()) {
            ty.set(lhs.get_type()).expect("TODO: panic message");
        } else {
            ty.set(AstType::Invalid).expect("TODO: panic message");
        }
        ty.get().unwrap().clone()
    }

    pub fn annotate_type_information(
        &mut self,
        type_data: &TypeInformation,
        locals: &HashMap<String, AstType>,
    ) -> AstType {
        match self {
            AstExpression::NumberLiteral { content, ty, .. } => {
                if content.contains(".") {
                    if let Ok(v) = f32::from_str(&content) {
                        ty.set(AstType::Float32).expect("type must not be set");
                    } else {
                        ty.set(AstType::Float64).expect("type must not be set");
                    }
                } else {
                    if let Ok(v) = i32::from_str(&content) {
                        ty.set(AstType::Int32).expect("type must not be set");
                    } else {
                        ty.set(AstType::Int64).expect("type must not be set");
                    }
                }
                
                ty.get().cloned().unwrap()
            }
            AstExpression::StringLiteral { content, ty, .. } => {
                ty.set(AstType::Structure("std::string".to_string()))
                    .expect("type must not be set");
                ty.get().cloned().unwrap()
            }
            AstExpression::VariableLiteral { content, ty, .. } => {
                ty.set(locals.get(content).expect("variable must exist").clone())
                    .expect("type must exist");
                ty.get().unwrap().clone()
            }
            AstExpression::PathLiteral(_) => {
                todo!()
            }
            AstExpression::ArrayLiteral { .. } => {
                todo!()
            }
            AstExpression::StructureLiteral { .. } => {
                todo!()
            }
            AstExpression::TypeLiteral { .. } => {
                todo!()
            }
            AstExpression::Add { lhs, rhs, ty, .. } => 
                Self::annotate_binop(lhs, rhs, ty, type_data, locals),
            AstExpression::Sub { lhs, rhs, ty, .. } => 
                Self::annotate_binop(lhs, rhs, ty, type_data, locals),
            AstExpression::Mul { lhs, rhs, ty, .. } => 
                Self::annotate_binop(lhs, rhs, ty, type_data, locals),
            AstExpression::Div { lhs, rhs, ty, .. } => 
                Self::annotate_binop(lhs, rhs, ty, type_data, locals),
            AstExpression::Mod { lhs, rhs, ty, .. } => 
                Self::annotate_binop(lhs, rhs, ty, type_data, locals),
            AstExpression::Invoke { .. } => {
                todo!()
            }
            AstExpression::Index { .. } => {
                todo!()
            }
        }
    }
}
