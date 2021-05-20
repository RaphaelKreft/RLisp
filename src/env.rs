use crate::types::{error, RlErr, RlReturn, RlType};
use crate::stdlib::core;
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

// Define RlEnv Type for convenience: the Env is wrapped in a Rc object, which allows easy referencing
pub type RlEnv = Rc<Env>;

#[derive(Clone, Debug)]
pub struct Env {
    env: RefCell<HashMap<String, RlType>>,
    outer: Option<RlEnv>,
}

pub(crate) fn init_global() -> RlEnv {
    let defenv = new_env(None);
    for (key, func) in core() {
        set(&defenv, key.to_string(), func);
    }
    return defenv.clone();
}

pub fn new_env(outer: Option<RlEnv>) -> RlEnv{
    return Rc::new(Env {
        env: RefCell::new(HashMap::new()),
        outer,
    });
}

pub fn new_env_bound(outer: Option<RlEnv>, names: Vec<RlType>, targets: Vec<RlType>) -> Result<RlEnv, RlErr> {
    let env = new_env(outer);
    return if names.len() != targets.len() {
        Err(error("Error: Number of arguments are not matching!"))
    } else {
        for (i, name) in names.iter().enumerate() {
            match name {
                RlType::Symbol(s) => set(&env, s.to_string(), targets[i].clone()),
                _ => return Err(error("Error: In self defined functions, Parameter names must be Symbols")),
            }
        }
        Ok(env.clone())
    }
}

pub(crate) fn set(envr: &RlEnv, symbol: String, expr: RlType) {
    envr.env.borrow_mut().insert(symbol.clone(), expr.clone());
}

pub fn search(envr: &RlEnv, key: String) -> RlReturn {
    match envr.env.borrow().get(&key) {
        None => match &envr.outer {
            Some(x) => search(x, key),
            None => Err(error(&format!("Symbol {} not found", key))),
        }
        Some(value) => Ok(value.clone()),
    }
}


