/*
printer.rs: Holds functions to transform an Sexpression (AST) the Evaluator works with
            back to a string.
*/

use super::types::{Sexpression};


pub fn print_str(expression: Sexpression) -> String {
    let string: String = print_str_rec(expression);
    return string;
}

fn print_str_rec(expression: Sexpression) -> String {
    match expression {
        Sexpression::Atom(value) => return String::from(value.value),
        Sexpression::List(vec) => {
            let itered: Vec<String> = vec.into_iter().map(print_str_rec).collect();
            let owned: String = format!("({})", itered.join(""));
            return owned;}
    }
}

