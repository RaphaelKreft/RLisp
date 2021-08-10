/*
eval.rs: holds the functionality of the evaluator of the Interpreter!

The module "eval" is a submodule of "main" and contains the two functions eval and apply which are
the core of the Interpreter. "eval" is the Interface to the Evaluator and is everything that needs
to be called to evaluate an expression.
*/

// load important functionality of other sibling-modules
use super::env::{new_env, search, set, RlEnv};
use super::types::{error, RlReturn, RlType};
use crate::env::new_env_bound;

// load needed Rust modules
use std::rc::Rc;
use crate::choices::{RlChoices, Choices};

/**
    Is the core function of the Interpreter, it takes an AST and tries to evaluate it.
    1. Check if given AST is a List
        1.1. If its a List, look what the first element of the List is
            1.1.1 If it's a special form, act accordingly(special forms have individual, non
                  standard evaluation behaviour)
            1.1.2. If It's no special form treat the first argument as function, treat the rest of
                   the elements as arguments of this function -> first evaluate arguments and then
                   apply to function(that's normal evaluation behaviour)
        1.2 If its an empty list just return (nothing to evaluate)
    2. If given AST is no List, then it's atomic
        2.1 Symbols will be looked up in the environment
        2.2 Integers, Bool, Nil and Strings are self-evaluating

    Arguments:  expression - Abstract Syntax Tree(AST) that represents the expression to evaluate
                environment - the environment the expression is evaluated in
    Returns:    of type RlReturn - in case of an Error, is RLError otherwise the resulting AST (result of the whole evaluation)
*/
pub fn eval(expression: RlType, environment: RlEnv, mut choices: RlChoices) -> RlReturn {
    match expression.clone() {
        // If given expression is a List
        RlType::List(content) => {
            // if list is empty return empty list back unchanged
            return if content.len() == 0 {
                Ok(expression)
            } else {
                // if list is not empty first check first element if it is Symbol triggering special form
                let switcher = &content[0];
                match switcher {
                    // quote special form: takes exactly one argument and don't evaluates it
                    RlType::Symbol(s) if s == "quote" => Ok(content[1].clone()),
                    // eval special form: takes exactly one argument and evaluates is (needed for Homoiconicity)
                    RlType::Symbol(s) if s == "eval" => {
                        eval(content[1].clone(), environment.clone(), choices)
                    }
                    // cond special form: takes a list of pairs. Each pair has a predicate and an according
                    //                    expression. Predicates are evaluated in order and the expression for
                    //                    the first predicate to be true is evaluated.
                    RlType::Symbol(s) if s == "cond" => {
                        let pairs = content[1..].to_vec().clone();
                        // iterate over pairs
                        for pair in pairs.iter() {
                            match pair {
                                // if we have valid pair evaluate predicate
                                RlType::List(l) if l.len() == 2 => {
                                    match eval(l[0].clone(), environment.clone(), choices.clone())? {
                                        RlType::Bool(true) => {
                                            return eval(l[1].clone(), environment.clone(), choices.clone());
                                        }
                                        _ => {
                                            continue;
                                        }
                                    }
                                },
                                // else pattern is invalid
                                _ => return Err(error("Error: Wrong pattern for cond")),
                            }
                        }
                        // if no predicate is evaluated to be true, return nil
                        return Ok(RlType::Nil);
                    }
                    // define special form: takes a symbol-name and a target expression and maps the
                    //                      symbol-name to the (evaluated)expression in current
                    //                      environment. if symbol-name already defined, target is overwritten.
                    RlType::Symbol(s) if s == "define" => {
                        return if content[1..].len() != 2 {
                            Err(error("Error: define takes exactly 2 ars"))
                        } else {
                            let key = match &content[1] {
                                RlType::Symbol(s) => s.to_string(),
                                _ => return Err(error("first arg of define must be a symbol")),
                            };
                            let target = eval(content[2].clone(), environment.clone(), choices.clone())?;
                            set(&environment, key.clone(), target.clone());
                            Ok(target)
                        }
                    }
                    // let special form: takes a list of pairs and an expression. The list of pairs
                    //                   contains (symbol, value) pairs that will be defined in the
                    //                   expression that was given as second environment.
                    //                   Creates a new environment with the bindings and evaluates expression.
                    RlType::Symbol(s) if s == "let" => {
                        return if content[1..].len() != 2 {
                            Err(error("Error: let takes exactly 2 args!"))
                        } else {
                            // check if first argument is a list
                            let bindings_list = match &content[1] {
                                RlType::List(l) => Ok(l),
                                _ => Err(error("Error: Arguments of binding lists must be pairs!")),
                            }?;
                            // create new sub-environment with current environment as outer environment
                            let new_env = new_env(Some(environment));
                            // iterate over present binding pairs in the given List
                            for binding in bindings_list.iter() {
                                // check if element is a pair
                                let b = match &binding {
                                    RlType::List(l) if l.len() == 2 => Ok(l),
                                    _ => Err(error(
                                        "Error: bindings in let needs to be lists of len 2",
                                    )),
                                }?;
                                // check if first element of pair is a symbol-name
                                let key = match &b[0] {
                                    RlType::Symbol(s) => s.to_string(),
                                    _ => return Err(error("first arg of define must be a symbol")),
                                };
                                // map symbol to evaluated value in the new environment
                                set(&new_env, key, eval(b[1].clone(), new_env.clone(), choices.clone())?);
                            }
                            // Evaluate body with new environment
                            eval(content[2].clone(), new_env.clone(), choices.clone())
                        };
                    }
                    // load special form: takes exactly one argument which is a string. This string
                    //                    will be treated as filename. Try to load and evaluate content
                    //                    of the file using the load() function in main.rs
                    RlType::Symbol(s) if s == "load" => {
                        // check if we have exactly one argument
                        if content[1..].len() != 1 {
                            return Err(error("load needs exactly one argument which is a string"));
                        }
                        // check if filename is a string
                        let filename = match &content[1] {
                            RlType::String(s) => s,
                            _ => return Err(error("load a string as argument!")),
                        };
                        // use load() in main.rs to process file
                        super::load(filename, environment.clone(), false);
                        // return nil since something needs to be returned
                        Ok(RlType::Nil)
                    }
                    // do special form: takes a list of expressions, evaluates them in-order and
                    //                  returns the value of the last expression evaluated.
                    RlType::Symbol(s) if s == "do" => {
                        // evaluate every expression except the last one
                        for expression in content[1..content.len() - 1].iter() {
                            let _ = eval(expression.clone(), environment.clone(), choices.clone());
                        }
                        // evaluate last expression and return its value
                        return eval(
                            content.last().unwrap_or(&RlType::Nil).clone(),
                            environment.clone(), choices.clone()
                        );
                    }
                    // lambda special form: takes two arguments, a list of formal arguments and an expression
                    //                      lambda then creates a function in which the arguments are bound
                    //                      to the according symbols in the given expression(=body of function)
                    RlType::Symbol(s) if s == "lambda" => {
                        // check if we have a list and an expression
                        match (content[1].clone(), content[2].clone()) {
                            (RlType::List(l1), body) => {
                                // return function object. Stores environment at time of creation
                                Ok(RlType::SelfDefinedFunc {
                                    env: environment,
                                    params: Rc::new(l1),
                                    body: Rc::new(body)
                                })
                            },
                            _ => Err(error("Error: lambda takes a list of parameters and an s-expression as body!"))
                        }
                    }
                    RlType::Symbol(s) if s == "amb" => {
                        // amb special form: first check if parameters were given if yes start new
                        // choices tree if not call next choice execution
                        if content.len() == 1 {
                            // call next choice execution
                            choices.next_choice()
                        } else {
                            // else start new choices tree TODO. maybe other structure to support nested ambs etc -> This is just a first approach
                            let given_choices = content[1..].to_vec();
                            choices = Choices::new_choices(given_choices, Option::from(choices.clone()))
                        }
                    }
                    _ => {
                        // Else evaluate every subexpression of the list and apply
                        let mut evaluated = Vec::new();
                        for element in content.iter() {
                            evaluated.push(eval(element.clone(), environment.clone(), choices.clone())?);
                        }
                        apply(evaluated)
                    }
                }
            };
        }
        // If given expression is no List but a Symbol, look up symbol in environment
        RlType::Symbol(s) => Ok(search(&environment, s)?),
        // Else given expression is self-evaluating
        _ => Ok(expression.clone()),
    }
}

/**
    This function is a helper for the eval function. It takes a list of expressions(that are already
    evaluated), treat the first expression as function and apply the given expressions to the function.
    Here we can ignore case of empty list because that case is caught in eval.

    Arguments:  args - the list of expressions
    Returns:    The value evaluated by the function application or an Error.
*/
pub fn apply(args: Vec<RlType>) -> RlReturn {
    let func = args[0].clone();
    // check if first argument is a function
    match func {
        // if its a function that is defined in Rust(part of the StdLib) just call it with the arguments
        RlType::Func(i) => i(args[1..].to_vec()),
        // if its a self defined function(in RLisp), evaluate the function body after binding parameters
        RlType::SelfDefinedFunc {
            env: stored_env,
            params: temp_params,
            body: temp_body,
        } => {
            let params = &*temp_params;
            let body = &*temp_body;
            // create function environment and bind given parameters to formal arguments of function
            let function_environment =
                new_env_bound(Some(stored_env.clone()), params.clone(), args[1..].to_vec())?;
            // then evaluate function body with new environment
            eval(body.clone(), function_environment.clone(), choices.clone())
        }
        _ => Err(error("Expected Function to apply!")),
    }
}
