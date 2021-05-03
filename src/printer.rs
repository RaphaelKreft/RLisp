/*
printer.rs: Holds functions to transform an Sexpression (AST) the Evaluator works with
            back to a string.
*/

use super::types::{Sexpression, Atom};


pub fn print_str(expression: Sexpression) -> String {
    let mut string: String = String::new();
    string = print_str_rec(Sexpression);
    return string;
}

fn print_str_rec(expression: Sexpression) -> String {
    match expression {
        Sexpression::Atom => return String::from(expression),
        Sexpression::List => return "(" + expression.into_iter().map(print_str_rec).collect() + ")",
    }
}
