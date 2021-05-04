/*
types.rs: Holds the types of the abstract syntax tree the parser works with
*/

use super::utils;

// An Sexpression is either an Atom or a List of Sexpressions
pub enum Sexpression {
    Atom(Atom),
    List(Vec<Sexpression>),
}

pub enum AtomType {
    Symbol,
    Int,
    Float,
}

pub struct Atom {
    pub typ: AtomType,
    pub value: String 
}


pub fn is_atom(expr: Sexpression) -> bool {
    match expr {
        Sexpression::Atom(_i) => true,
        _ => false,
    }
}

pub fn get_value_atom(expr: Sexpression) -> Atom {
    match expr {
        Sexpression::Atom(i) => i,
        _ => Atom::new(&String::new()),
    }
}

pub fn get_value_list(expr: Sexpression) -> &Vec<Sexpression> {
    match expr {
        Sexpression::List(i) => &i,
        _ => Vec::new(),
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
