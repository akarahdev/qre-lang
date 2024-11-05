use std::collections::HashMap;
use crate::frontend::parser::ast::{AstType, PathData};

#[derive(Debug, Clone)]
pub enum ProgramType {
    Structure {
        name: String,
        fields: Vec<(String, AstType)>
    },
    Function {
        name: String,
        arguments: Vec<(String, AstType)>,
        returns: AstType
    }
}

#[derive(Debug, Clone)]
pub struct TypeInformation {
    pub names: HashMap<String, ProgramType>
}