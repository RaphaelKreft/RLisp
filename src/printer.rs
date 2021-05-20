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
        RlType::Int(value) => format!("{}", value),
        RlType::Symbol(i) => format!("{}", i),
        RlType::String(i) => format!("'{}'", i),
        RlType::List(vec) => {
            let iter: Vec<String> = vec.into_iter().map(print_str_rec).collect();
            let owned: String = format!("({})", iter.join(" "));
            owned
        }
        RlType::Bool(b) => if b {String::from("#t")} else {String::from("#f")}
        RlType::Func(_) | RlType::SelfDefinedFunc { .. } => String::from("#function"),
        RlType::Nil => String::from("#nil"),
    }
}

