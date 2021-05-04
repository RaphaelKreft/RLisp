/*
printer.rs: Holds functions to transform an RlType (AST) the Evaluator works with
            back to a string.
*/

use super::types::{RlType};


pub fn print_str(expression: RlType) -> String {
    let string: String = print_str_rec(expression);
    return string;
}

fn print_str_rec(expression: RlType) -> String {
    return match expression {
        RlType::Atom(value) => format!(" {} ", value.value),
        RlType::List(vec) => {
            let itered: Vec<String> = vec.into_iter().map(print_str_rec).collect();
            let owned: String = format!("({})", itered.join(""));
            owned
        }
    }
}

