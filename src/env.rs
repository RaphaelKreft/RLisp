use crate::types::{error, RlErr, RlReturn, RlType};
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
    set(&defenv,"car".to_string(), car());
    set(&defenv,"cdr".to_string(), cdr());
    set(&defenv,"cons".to_string(), cons());
    set(&defenv,"+".to_string(), integer_arithmetics("+"));
    set(&defenv,"-".to_string(), integer_arithmetics("-"));
    set(&defenv,"*".to_string(), integer_arithmetics("*"));
    set(&defenv,"/".to_string(), integer_arithmetics("/"));
    set(&defenv,"eq".to_string(), equals());
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

// Define The Functions for Integer arithmetics
fn integer_arithmetics(sym: &str) -> RlType {
    match sym {
        "+" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            return Ok(RlType::Int(x.to_vec().iter().sum()));
        }),
        "/" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x[0] == 0 {
                Err(error("cant divide by zero"))
            } else {
                let divisor: i64 = x[1..].to_vec().iter().sum();
                Ok(RlType::Int(x[0] / divisor))
            }
        }),
        "*" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            return Ok(RlType::Int(x.to_vec().iter().product()));
        }),
        _ => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            let neg: i64 = x[1..].to_vec().iter().sum();
            Ok(RlType::Int(x[0] - neg))
        }),
    }
}

// Function to check whether the vector just consists Integers
fn check_int_vector(vec: Vec<RlType>) -> Result<Vec<i64>, RlErr> {
    let mut new_vec = Vec::new();
    for element in vec.iter() {
        match element {
            RlType::Int(i) => new_vec.push(i.clone()),
            _ => return Err(error("Wrong kind of arguments!")),
        }
    }
    if new_vec.len() < 2 {
        return Err(error("not enough arguments for calculation"));
    }
    return Ok(new_vec);
}

// List operations are defined in separate functions for style reasons

// car returns the first element of a given list
fn car() -> RlType {
    return RlType::Func(|a| match &a[0] {
        RlType::List(l) => {
            return if l.len() != 2 {
                Err(error("car needs a list of len 2"))
            } else {
                Ok(l.get(0).unwrap().clone())
            }
        }
        _ => Err(error("car expects a list!")),
    });
}

// cdr returns the second part(the rest) of a given list
fn cdr() -> RlType {
    return RlType::Func(|a| match &a[0] {
        RlType::List(l) => {
            return if l.len() != 2 {
                Err(error("cdr needs a list of len 2"))
            } else {
                Ok(l.get(1).unwrap().clone())
            }
        }
        _ => Err(error("cdr expects a list!")),
    });
}

fn cons() -> RlType {
    return RlType::Func(|a| {
        return if a.len() != 2 {
            Err(error("cons needs a list of len 2"))
        } else {
            Ok(RlType::List(a))
        };
    });
}

fn equals() -> RlType {
    return RlType::Func(|a| {
        return if a.len() != 2 {
            Err(error("eq takes exactly 2 args"))
        } else {
            Ok(RlType::Bool(a[0] == a[1]))
        };
    });
}
