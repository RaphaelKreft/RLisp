use crate::types::{error, RlErr, RlReturn, RlType};
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;


pub type RlEnv = Rc<Env>;

#[derive(Clone)]
pub struct Env {
    env: RefCell<HashMap<String, RlType>>,
    outer: Option<RlEnv>,
}

pub(crate) fn init_global() -> RlEnv {
    let defenv = new_env(None);
    set(defenv.clone(),"car".to_string(), car());
    set(defenv.clone(),"cdr".to_string(), cdr());
    set(defenv.clone(),"cons".to_string(), cons());
    set(defenv.clone(),"+".to_string(), integer_arithmetics("+"));
    set(defenv.clone(),"-".to_string(), integer_arithmetics("-"));
    set(defenv.clone(),"*".to_string(), integer_arithmetics("*"));
    set(defenv.clone(),"/".to_string(), integer_arithmetics("/"));
    set(defenv.clone(),"eq".to_string(), equals());
    return defenv.clone();
}

pub fn new_env(outer: Option<RlEnv>) -> RlEnv{
    return Rc::new(Env {
        env: RefCell::new(HashMap::new()),
        outer,
    });
}

pub(crate) fn set(envr: RlEnv, symbol: String, expr: RlType) {
    envr.env.borrow_mut().insert(symbol.clone(), expr.clone());
}

pub(crate) fn get(envr: RlEnv, key: String) -> RlReturn {
    match envr.env.borrow().get(&key) {
        None => Err(error(&format!("Symbol {} not found", key))),
        Some(value) => Ok(value.clone()),
    }
}

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

// List operations are defined in separate functions for style reasons TODO: find better solution & List unpacking with cons/car/cdr?!
fn car() -> RlType {
    return RlType::Func(|a| match &a[0] {
        RlType::List(l) => {
            return if l.len() != 2 {
                Err(error("cdr needs a list of len 2"))
            } else {
                Ok(l.get(0).unwrap().clone())
            }
        }
        _ => Err(error("cdr expects a list!")),
    });
}

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
