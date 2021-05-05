/*
types.rs: Holds the types of the abstract syntax tree the parser works with
*/

use crate::types::RlErr::ErrString;
use std::fmt;

// This type is needed for error handling since in rust this is just possible via return values
pub type RlReturn = Result<RlType, RlErr>;
//pub type RlEnv = HashMap<String, RlType>;

// An RlType is either an Atom or a List of RlType
#[derive(Debug, Clone)]
pub enum RlType {
    Int(i64),
    Symbol(String),
    String(String),
    Func(fn(Vec<RlType>) -> RlReturn),
    List(Vec<RlType>),
}

// A Type to define Errors
#[derive(Debug)]
pub enum RlErr {
    ErrString(String),
}

// defines which of the types are an atom
pub fn is_atom(expr: RlType) -> bool {
    match expr {
        RlType::Int(_i) => true,
        RlType::Symbol(_i) => true,
        _ => false,
    }
}

// Implement this trait so that errors are shown nicely
impl fmt::Display for RlErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrString(i) => write!(f, "{}", i),
        }
    }
}

// helper to create a new RlReturn value as Error
pub fn error(str: &str) -> RlErr {
    return ErrString(String::from(str));
}
