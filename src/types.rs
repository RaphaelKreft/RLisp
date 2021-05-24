/*
types.rs: Holds the types of the abstract syntax tree the parser and evaluator work with
*/

use crate::env::RlEnv;
use crate::types::RlErr::ErrString;
use std::fmt;
use std::rc::Rc;

/// This type is needed for error handling since in rust that's just possible via return values
pub type RlReturn = Result<RlType, RlErr>;

/// An RlType is either an Atom or a List of RlType
#[derive(Debug, Clone)]
pub enum RlType {
    Int(i64),
    Bool(bool),
    Symbol(String),
    String(String),
    Func(fn(Vec<RlType>) -> RlReturn),
    SelfDefinedFunc {
        env: RlEnv,
        params: Rc<Vec<RlType>>,
        body: Rc<RlType>,
    },
    List(Vec<RlType>),
    Nil,
}

/// A Type to define Errors
#[derive(Debug)]
pub enum RlErr {
    ErrString(String),
}

/// defines which of the types are an atom: Int, Symbol, String, Nil, Bool, Empty list
pub fn is_atom(expr: RlType) -> bool {
    match expr {
        RlType::Int(_i) => true,
        RlType::Symbol(_i) => true,
        RlType::Nil => true,
        RlType::Bool(_i) => true,
        RlType::String(_i) => true,
        RlType::List(l) if l.len() == 0 => true,
        _ => false,
    }
}

/// Implement the display trait so that errors are shown nicely
impl fmt::Display for RlErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrString(i) => write!(f, "{}", i),
        }
    }
}

/// Implement the PartialEq trait for the RlType so that comparisons are possible. This is mostly used
/// for the "eq" functionality of RLisp
impl PartialEq for RlType {
    fn eq(&self, other: &RlType) -> bool {
        match (self, other) {
            (RlType::Int(ref a), RlType::Int(ref b)) => a == b,
            (RlType::Bool(ref a), RlType::Bool(ref b)) => a == b,
            (RlType::Symbol(ref a), RlType::Symbol(ref b)) => a == b,
            (RlType::Nil, RlType::Nil) => true,
            (RlType::List(ref a), RlType::List(ref b)) => a == b,
            (RlType::String(ref a), RlType::String(ref b)) => a == b,
            _ => false,
        }
    }
}

/// helper to create a new RlError, this is just for convinience
pub fn error(str: &str) -> RlErr {
    return ErrString(String::from(str));
}
