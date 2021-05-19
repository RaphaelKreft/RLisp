/*
eval.rs: holds the functionality of the evaluator of the Interpreter!
*/

use super::types::{RlType, RlReturn, error};
use super::env::{RlEnv, new_env, search, set};
use crate::env::new_env_bound;
use std::rc::Rc;

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
                        if content[1..].len() < 2 {
                            return Err(error("Error: cond expects at least 2 args"));
                        }
                        let conditional = eval(content[1].clone(), environment.clone())?;
                        match conditional {
                            RlType::Bool(false) => {
                                if content[1..].len() == 2 {
                                    Ok(RlType::Nil)
                                } else {
                                    eval(content[3].clone(), environment.clone())
                                }
                            },
                            RlType::Bool(true) if content[1..].len() >= 2 => {
                                eval(content[2].clone(), environment.clone())
                            },
                            // Return nil if first arg was no conditional or no true fork is given
                            _ => Ok(RlType::Nil)
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
                            set(&environment, key.clone(), target.clone());
                            Ok(target)
                        }
                    },
                    RlType::Symbol(s) if s == "let" => {
                        return if content[1..].len() != 2 {
                            Err(error("Error: let takes exactly 2 ars"))
                        } else {
                            // load definitions in new created sub-environment
                            let bindings_list = match &content[1] {
                                RlType::List(l) => Ok(l),
                                _ => Err(error("Error: Arguments of binding lists must be pairs!")),
                            }?;
                            let new_env = new_env(Some(environment));
                            // iterate over present binding pairs in List
                            for binding in bindings_list.iter() {
                                let b = match &binding {
                                    RlType::List(l) => Ok(l),
                                    _ => Err(error("Error: bindings in let needs to be lists of len 2"))
                                }?;
                                let key = match &b[0] {
                                    RlType::Symbol(s) => s.to_string(),
                                    _ => return Err(error("first arg of define must be a symbol")),
                                };
                                set(&new_env, key, eval(b[1].clone(), new_env.clone())?);
                                }

                            // Evaluate body with new environment
                            eval(content[2].clone(), new_env.clone())
                        }
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
                        Ok(RlType::Nil)
                    },
                    RlType::Symbol(s) if s == "lambda" => {
                        match (content[1].clone(), content[2].clone()) {
                            (RlType::List(l1), body) => {
                                Ok(RlType::SelfDefinedFunc {
                                    env: environment,
                                    params: Rc::new(l1),
                                    body: Rc::new(body)
                                })
                            },
                            _ => Err(error("Error: lambda takes a list of parameters and an sexpression as body!"))
                        }
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
        RlType::Symbol(s) => Ok(search(&environment, s)?),
        _ => Ok(expression.clone())
    }
}

pub fn apply(args: Vec<RlType>) -> RlReturn{
    let func = args[0].clone();
    match func {
        RlType::Func(i) => i(args[1..].to_vec()),
        RlType::SelfDefinedFunc {
            env: stored_env,
            params: temp_params,
            body: temp_body,
        } => {
            let params = &*temp_params;
            let body = &*temp_body;
            let function_environment = new_env_bound(Some(stored_env.clone()), params.clone(), args[1..].to_vec())?;
            eval(body.clone(), function_environment.clone())
        },
        _ => Err(error("Expected Function to apply!")),
    }
}

