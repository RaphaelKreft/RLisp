use crate::printer::print_str;
use crate::types::{error, is_atom, RlErr, RlType};

pub fn core() -> Vec<(&'static str, RlType)> {
    vec![
        ("car", car()),
        ("cdr", cdr()),
        ("cons", cons()),
        ("+", integer_arithmetics("+")),
        ("-", integer_arithmetics("-")),
        ("*", integer_arithmetics("*")),
        ("/", integer_arithmetics("/")),
        ("eq?", equals()),
        ("nil?", type_check("nil")),
        ("number?", type_check("int")),
        ("list?", type_check("list")),
        (
            "println",
            RlType::Func(|a| {
                println!("{}", print_str(RlType::List(a)));
                Ok(RlType::Nil)
            }),
        ),
        (
            "atom?",
            RlType::Func(|a| Ok(RlType::Bool(is_atom(a[0].clone())))),
        ),
    ]
}

fn type_check(typ: &str) -> RlType {
    match typ {
        "nil" => RlType::Func(|arg| {
            Ok(RlType::Bool(match arg[0] {
                RlType::Nil => true,
                _ => false,
            }))
        }),
        "int" => RlType::Func(|arg| {
            Ok(RlType::Bool(match arg[0] {
                RlType::Int(..) => true,
                _ => false,
            }))
        }),
        _ => RlType::Func(|arg| {
            Ok(RlType::Bool(match arg[0] {
                RlType::List(..) => true,
                _ => false,
            }))
        }),
    }
}

/// Define The Functions for Integer arithmetics
fn integer_arithmetics(sym: &str) -> RlType {
    match sym {
        "+" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            return Ok(RlType::Int(x.to_vec().iter().sum()));
        }),
        "/" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            let divisor: i64 = x[1..].to_vec().iter().sum();
            if divisor == 0 {
                Err(error("cant divide by zero"))
            } else {
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

/// Function to check whether the vector just consists Integers
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

/// car returns the first element of a given list
fn car() -> RlType {
    return RlType::Func(|a| match &a[0] {
        RlType::List(l) => {
            println!("{:?}", a);
            return if l.len() < 1 {
                Err(error("car needs a list of min len 1"))
            } else {
                Ok(l.get(0).unwrap().clone())
            }
        }
        _a => Err(error(&*format!("car expects a list! but got {:?}", _a))),
    });
}

/**
    cdr returns the second part(the rest) of a given list if list has one element return NIL as
    this is the return value of cdr on a one element list in normal lisp (lists are terminated with NIL)
*/
fn cdr() -> RlType {
    return RlType::Func(|a| match &a[0] {
        RlType::List(l) => {
            return if l.len() < 1 {
                Err(error("cdr needs a list with min len 1!"))
            } else if l.len() == 1 {
                Ok(RlType::List(vec![]))
            } else {
                Ok(RlType::List(l[1..].to_vec().clone()))
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
