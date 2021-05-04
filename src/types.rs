/*
types.rs: Holds the types of the abstract syntax tree the parser works with
*/

use super::utils;
use crate::types::RlErr::ErrString;
use std::fmt;

// This type is needed for error handling since in rust this is just possible via return values
pub type RlReturn = Result<RlType, RlErr>;

// An RlType is either an Atom or a List of RlType
#[derive(Debug)]
pub enum RlType {
    Atom(Atom),
    List(Vec<RlType>),
}

// A Type to define Errors
#[derive(Debug)]
pub enum RlErr {
    ErrString(String),
}

#[derive(Debug)]
pub enum AtomType {
    Symbol,
    Int,
    Float,
}

#[derive(Debug)]
pub struct Atom {
    pub typ: AtomType,
    pub value: String 
}

pub fn is_atom(expr: RlType) -> bool {
    match expr {
        RlType::Atom(_i) => true,
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

impl Atom {
    pub fn new(value: &String) -> Atom {
        if utils::string_holds_integer(value.to_string()) {
            return Atom{value: value.to_string(), typ: AtomType::Int};
        } else if utils::string_holds_float(value.to_string()) {
            return Atom{value: value.to_string(), typ: AtomType::Float};
        } else {
            return Atom{value: value.to_string(), typ: AtomType::Symbol};
        }
    }
}

// helper to create a new RlReturn value as Error
pub fn error(str: &str) -> RlReturn {
    return Err(ErrString(String::from(str)));
}
