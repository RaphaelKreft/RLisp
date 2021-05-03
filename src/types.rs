/*
types.rs: Holds the types of the abstract syntax tree the parser works with
*/

use super::utils;

enum Atom_Type {
    S(String),
    I(i64),
    F(f64),
 //   N(Nil),
    T(bool),
 //   F(bool)
}

pub struct Atom {
    typ: Atom_Type;
    value: String;
}

// An Sexpression is either an Atom or a List of Sexpressions
pub enum Sexpression {
    Atom(Atom),
    List(Vec<Sexpression>)
}

impl Atom {
    fn new(value: String) -> Atom {
        if utils::sting_holds_integer(value) {

        } else if utils::sting_holds_float(value) {

        } else {

        }
    }
}

/*
impl Sexpression {
    fn isAtom(expression: Sexpression) -> bool {
        return expression == Sexpression::Atom;
    }
    
    fn isList(expression: Sexpression) -> bool {
        return expression == Sexpression::List;
    }
}
*/