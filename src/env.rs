/*
env.rs: This module contains the definition of an environment in RLisp. An environment is a map,
        that contains mappings from symbols to expressions. An environment is used by the Evaluator
        tho evaluate expressions. See eval.rs.
 */

// load functionality from sibling modules
use crate::stdlib::core;
use crate::types::{error, RlErr, RlReturn, RlType};
// load needed Rust modules
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// Define RlEnv Type for convenience: the Env is wrapped in a Rc object(Smart Pointer),
/// which allows easy referencing
pub type RlEnv = Rc<Env>;

/// This structure represents an environment/closure, it holds a Hashmap with the mapping of symbols to
/// expressions as well as a pointer to an outer environment
#[derive(Clone, Debug)]
pub struct Env {
    env: RefCell<HashMap<String, RlType>>,
    outer: Option<RlEnv>,
}

/**
    Creates a new environment and loads all the functions defined in the stdlib into this environment.
    Then returns this environment which will be used as the initial global environment.

    Returns:    The new environment (Type RlEnv)
*/
pub(crate) fn init_global() -> RlEnv {
    // create new RlEnv Instance using new_env()
    let defenv = new_env(None);
    // load definitions/mappings from stdlib module and set them in the environment
    for (key, func) in core() {
        set(&defenv, key.to_string(), func);
    }
    return defenv.clone();
}

/**
    Creates a new RlEnv Instance with an optional pointer to an outer environment.

    Arguments:  outer - is an option for a pointer to an outer environment (None if not given)
    Returns:    new RlEnv instance
*/
pub fn new_env(outer: Option<RlEnv>) -> RlEnv {
    return Rc::new(Env {
        env: RefCell::new(HashMap::new()),
        outer,
    });
}

/**
    creates a new environment and directly set given key value pairs in the environment. Then
    returns the environment or an RlError if anything fails.

    Arguments:  outer - optional pointer to outer environment of new environment
                names - a list of symbol-names that will be mapped to expressions in new environment
                targets - the list of expressions matching the symbol-names in names
    Returns:    The new environment with the bindings or an Error
*/
pub fn new_env_bound(
    outer: Option<RlEnv>,
    names: Vec<RlType>,
    targets: Vec<RlType>,
) -> Result<RlEnv, RlErr> {
    // create new environment using new_env()
    let env = new_env(outer);
    // check if lengths of lists are matching
    return if names.len() != targets.len() {
        Err(error("Error: Number of arguments are not matching!"))
    } else {
        // iterate through names
        for (i, name) in names.iter().enumerate() {
            match name {
                // if name is a valid Symbol, set the symbol-name to matching expression in targets
                RlType::Symbol(s) => set(&env, s.to_string(), targets[i].clone()),
                _ => {
                    return Err(error(
                        "Error: In self defined functions, Parameter names must be Symbols",
                    ))
                }
            }
        }
        // Return new environment
        Ok(env.clone())
    };
}

/**
    Takes an environment, a symbol-name and an expression. Associate the symbol-name with the given
    expression in the given environment.

    Arguments:  environment - the environment to set the given key-value pair
                symbol - the symbolname to set
                expr - the expression to map the associate the symbol with
*/
pub(crate) fn set(environment: &RlEnv, symbol: String, expr: RlType) {
    environment.env.borrow_mut().insert(symbol.clone(), expr.clone());
}

/**
    Takes an environment and a symbol-name. Searches the environment for the symbol name.
    Start with the most inner environment and proceed with outer environments until symbol-name
    mapping is found. (leads to shadowing of variables) Returns the found mapping(value to symbol)
    or an Error if symbol not found.

    Arguments:  environment - the environment to search in
                key - the symbol to look for
*/
pub fn search(environment: &RlEnv, key: String) -> RlReturn {
    // look for symbol in current environment
    match environment.env.borrow().get(&key) {
        // if symbol not found check if there is an outer environment
        None => match &environment.outer {
            // if there is an outer environment search in outer environment
            Some(x) => search(x, key),
            // if not we are certain that symbol is not defined
            None => Err(error(&format!("Symbol {} not found", key))),
        },
        // if symbol was found, return it's value
        Some(value) => Ok(value.clone()),
    }
}
