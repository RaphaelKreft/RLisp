/*
eval.rs: holds the functionality of the evaluator of the Interpreter!
*/

use super::types::{RlType, RlReturn, error};
use super::env::RlEnv;

pub fn eval(expression: RlType, environment: RlEnv) -> RlReturn {
    match expression.clone() {
        RlType::List(content) => {
            // return empty list back unchanged
            return if content.len() == 0 {
                Ok(expression)
            } else {
                // Else evaluate every subexpression of the list and apply
                let mut evaluated = Vec::new();
                for element in content.iter() {
                    evaluated.push(eval(element.clone(), environment.clone())?);
                }
                apply(evaluated)
            }
        },
        RlType::Symbol(s) => Ok(environment.get(s)?),
        _ => Ok(expression.clone())
    }
}

pub fn apply(args: Vec<RlType>) -> RlReturn{
    let func = &args[0];
    match func {
        RlType::Func(i) => i(args[1..].to_vec()),
        _ => Err(error("Expected Function to apply!")),
    }
}

