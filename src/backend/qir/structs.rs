use crate::frontend::parser::ast::PathData;

/// Represents a header in QIR
pub enum QIRHeader {
    /// Always represents a Ptr
    GlobalVariable {
        name: PathData,
        ty: QIRType
    },
    Function {
        name: PathData,
        parameters: Vec<(QIRLocalVariable, QIRType)>,
        returns: QIRType,
        code_block: QIRBasicBlock,
    },
    /// A very low level interface.
    ///
    /// Field indexing is 0-based.
    ///
    /// The struct must have 2 u32's at the start, for type ID and reference count specifically.
    Struct {
        name: PathData,
        fields: Vec<QIRType>
    }
}

pub struct QIRBasicBlock {
    exprs: Vec<QIRExpression>
}

pub enum QIRExpression {
    /// Returns the new reference count.
    /// Increments the reference count by 1.
    Retain {
        ptr: Box<QIRExpression>
    },

    /// Returns the new reference count.
    /// Decrements the reference count by 1.
    Release {
        ptr: Box<QIRExpression>
    },

    /// Almost all AST forms will be abstracted into a function call.
    /// E.g 2 + 1 will be rewritten into `qre::i32::add(2, 1)`
    Invoke {
        name: PathData,
        arguments: Vec<QIRExpression>,
        return_type: QIRType
    },
    /// Always creates a QIRType::Ptr type value
    InstantiateStructure {
        name: PathData
    },
    /// Stores a value to a Ptr
    StoreToPtr {
        receiver: Box<QIRExpression>,
        output_type: QIRType,
        new_value: Box<QIRExpression>
    },
    /// Calculates a Ptr from a field of a structure Ptr
    GetFieldPtr {
        receiver: Box<QIRExpression>,
        ptr_type: Vec<QIRType>,
        output_type: QIRType,
        /// field 0 = type ID (u32)
        ///
        /// field 1 = refcount (u32)
        ///
        /// field >=2 = structure's fields in LLVM IR
        field: i32
    },
    /// Loads the value directly from a pointer.
    LoadFromPtr {
        ptr: Box<QIRExpression>
    },
    /// Executes the given basic block directly
    GotoBlock {
        block: QIRBasicBlock
    },
    /// Branch if a condition is true
    BranchIf {
        condition: Box<QIRExpression>,
        if_true: QIRBasicBlock,
        if_false: QIRBasicBlock,
        continuation: QIRBasicBlock,
    },
    /// Always yields a Ptr value, pointing to a local variable.
    GetLocalPtr {
        local: QIRLocalVariable
    },
    /// Always yields a Ptr value, pointing to a global variable.
    GetGlobalPtr {
        global: PathData
    }
}

pub struct QIRLocalVariable {

}

pub enum QIRType {
    Int32,
    Int64,
    Float32,
    Float64,
    Ptr,
    Void,
    Invalid,
    Union
}