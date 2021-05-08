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
                // if list is not empty first check first element if it is Symbol triggering special form
                let switcher = &content[0];
                match switcher {
                    RlType::Symbol(s) if s == "quote" => {
                        Ok(content[1].clone())
                    },
                    RlType::Symbol(s) if s == "eval" => {
                        eval(content[1].clone(), environment.clone())
                    },
                    RlType::Symbol(s) if s == "cond" => {
                        if content[1..].len() != 3 {
                            return Err(error("Error: cond expects 3 arguments"));
                        }
                        let conditional = eval(content[1].clone(), environment.clone())?;
                        match conditional {
                            RlType::Bool(b) => {
                                if b {
                                    eval(content[2].clone(), environment.clone())
                                } else {
                                    eval(content[3].clone(), environment.clone())
                                }
                            },
                            _ => Err(error("Error: cond's first arg must be a conditional"))
                        }
                    },
                    RlType::Symbol(s) if s == "define" => {
                        return if content[1..].len() != 2 {
                            Err(error("Error: define takes exactly 2 ars"))
                        } else {
                            let key = match &content[1] {
                                RlType::Symbol(s) => s.to_string(),
                                _ => return Err(error("first arg of define must be a symbol")),
                            };
                            let target = eval(content[2].clone(),  environment.clone())?;
                            super::env::set(environment.clone(), key.clone(), target.clone());
                            Ok(target)
                        }
                    },
                    RlType::Symbol(s) if s == "let" => {
                        eval(content[1].clone(), environment.clone())
                    },
                    RlType::Symbol(s) if s == "load" => {
                        if content[1..].len() != 1 {
                            return Err(error("load needs exactly one argument which is a string"));
                        }
                        let filename = match &content[1] {
                            RlType::String(s) => s,
                            _ => return Err(error("load a string as argument!")),
                        };
                        super::load(filename, environment);
                        Ok(RlType::String("Loaded file successful".to_string())) // TODO: maybe change to nil?
                    },
                    _ => {// Else evaluate every subexpression of the list and apply
                        let mut evaluated = Vec::new();
                        for element in content.iter() {
                            evaluated.push(eval(element.clone(), environment.clone())?);
                        }
                        apply(evaluated)
                    },
                }
            }
        },
        RlType::Symbol(s) => Ok(super::env::get(environment, s)?),
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

