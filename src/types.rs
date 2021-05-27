/*
types.rs: Holds the types of the Abstract-Syntax-Tree(AST) named RlType, the parser and evaluator work with.
          It also defines the Error-type and a ReturnType that is needed for Error-handling.
*/

// load needed sibling-modules
use crate::env::RlEnv;
use crate::types::RlErr::ErrString;

// load needed Rust-Functionality
use std::fmt;
use std::rc::Rc;

/// This type is needed for error handling since in rust that's just possible via return values
pub type RlReturn = Result<RlType, RlErr>;

/**
    RLType is the internal Data-Structure of RLisp, it represents the AST.
    Please find the README for further information on the types.
*/
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
    // Defines Error-type String
    ErrString(String),
}

/**
    Defines which of the types are an atom: Int, Symbol, String, Nil, Bool, Empty list.
    It takes an arbitrary expression and returns a Boolean whether given expression has atomic type.

    Arguments:  expr - expression of type RLType, that is to be checked
    Returns:    true if given type is atomic, false otherwise
*/
pub fn is_atom(expr: RlType) -> bool {
    match expr {
        RlType::Int(_i) => true,
        RlType::Symbol(_i) => true,
        RlType::Nil => true,
        RlType::Bool(_i) => true,
        RlType::String(_i) => true,
        //RlType::List(l) if l.len() == 0 => true,
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
/// for the "eq?" functionality of RLisp
impl PartialEq for RlType {
    /**
        takes reference to Another RLType and checks for equality
        1. check if types are matching
        2. If types have a value, check if values are matching

        Arguments:  self - reference to local RLType
                    other - reference to other RLType to compare with
        Returns:    bool whether equality is given or not.
    */
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

/**
    Helper to create ErrString-Instance

    Arguments:  str - String that should be the errormessage
    Returns:    new ErrString Instance
*/
pub fn error(str: &str) -> RlErr {
    return ErrString(String::from(str));
}
