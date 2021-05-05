use crate::types::{error, RlErr, RlReturn, RlType};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RlEnv {
    env: HashMap<String, RlType>,
    //    outer: Option<RlEnv>,
}

impl RlEnv {
    pub(crate) fn init_global() -> RlEnv {
        let mut env = RlEnv {
            env: HashMap::new(), /*outer: None*/
        };
        env.set("load".to_string(), load());
        env.set("car".to_string(), car());
        env.set("cdr".to_string(), cdr());
        env.set("cons".to_string(), cons());
        env.set("+".to_string(), integer_arithmetics("+"));
        env.set("-".to_string(), integer_arithmetics("-"));
        env.set("*".to_string(), integer_arithmetics("*"));
        env.set("/".to_string(), integer_arithmetics("/"));
        return env;
    }

    fn set(&mut self, symbol: String, expr: RlType) {
        self.env.insert(symbol, expr);
    }

    pub(crate) fn get(&self, key: String) -> RlReturn {
        match self.env.get(&key) {
            None => Err(error(&format!("Symbol {} not found", key))),
            Some(value) => Ok(value.clone()),
        }
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

// load method for in-evaluation-use
fn load() -> RlType {
    return RlType::Func(|a| {
        if a.len() != 1 {
            return Err(error("load needs exactly one argument which is a string"));
        }
        let filename = match &a[0] {
            RlType::String(s) => s,
            _ => return Err(error("load a string as argument!")),
        };
        return if let Ok(lines) = super::read_lines(filename) {
            // Iterate over lines and check if they are ok(Or EOF)
            for line in lines {
                if let Ok(input) = line {
                    if input.starts_with(";") {
                        continue;
                    }
                    super::rep_wrapper(&input);
                }
            }
            Ok(RlType::Symbol(String::from("--File loaded successfully!--")))
        } else {
            Err(error("there was a problem with your file!"))
        }
    });
}
