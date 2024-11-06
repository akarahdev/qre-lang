use crate::frontend::parser::ast::AstHeader;
use crate::frontend::typecheck::data::{ProgramType, TypeInformation};

impl AstHeader {
    pub fn gather_type_information(&self, info: &mut TypeInformation) {
        match self {
            AstHeader::Import(_) => {}
            AstHeader::Function {
                name,
                parameters,
                returns,
                code_block,
                locals,
            } => {
                info.names.insert(
                    name.name.clone(),
                    ProgramType::Function {
                        name: name.name.clone(),
                        arguments: parameters
                            .iter()
                            .map(|x| (x.1.clone(), x.0.clone()))
                            .collect(),
                        returns: returns.clone(),
                    },
                );
            }
            AstHeader::Struct { name, fields } => {
                info.names.insert(
                    name.name.clone(),
                    ProgramType::Structure {
                        name: name.name.clone(),
                        fields: fields.clone(),
                    },
                );
            }
        }
    }
}
