/*
stdlib.rs: holds the definition of all non-special form language atoms of RLisp that are defined in
           Rust language. Other modules can use core() function to get all these functions.
 */

// load needed sibling-modules
use crate::printer::{print_str_rec};
use crate::types::{error, is_atom, RlErr, RlType};

/**
    Is the Interface to the whole stdlib. It simply returns a mapping from symbol-names to RLisp
    Functions type:RLType::Func. This is the type of all functions defined in the host Language

    Returns: A vector that contains pairs: (symbol-name, function/expression)
*/
pub fn core() -> Vec<(&'static str, RlType)> {
    vec![
        ("car", car()),
        ("cdr", cdr()),
        ("cons", cons()),
        ("list", RlType::Func(|a| Ok(list(a)))),
        ("+", integer_arithmetics("+")),
        ("-", integer_arithmetics("-")),
        ("*", integer_arithmetics("*")),
        ("/", integer_arithmetics("/")),
        (">", integer_arithmetics(">")),
        ("<", integer_arithmetics("<")),
        ("eq?", equals()),
        ("%", modulo()),
        ("nil?", type_check("nil")),
        ("number?", type_check("int")),
        ("list?", type_check("list")),
        (
            "println",
            RlType::Func(|a| {
                if a.len() != 1 {return Err(error("println takes exactly one argument"))}
                println!("{}", print_str_rec(a[0].clone()));
                Ok(a[0].clone())
            }),
        ),
        (
            "atom?",
            RlType::Func(|a| Ok(RlType::Bool(is_atom(a[0].clone())))),
        ),
    ]
}

/**
    This function returns functions of type RlType::Func depending on the parameter. This function
    exists for readability.

    Arguments:  typ - a string that describes which function should be returned
    Returns:    a Function of type RlType::Func performing a type check
*/
fn type_check(typ: &str) -> RlType {
    // check which function is wanted
    match typ {
        // return function for nil-typecheck
        "nil" => RlType::Func(|arg| {
            Ok(RlType::Bool(match &arg[0] {
                RlType::Nil => true,
                RlType::List(l) if l.len() == 0 => true,
                _ => false,
            }))
        }),
        // return function for Integer-typecheck
        "int" => RlType::Func(|arg| {
            Ok(RlType::Bool(match arg[0] {
                RlType::Int(..) => true,
                _ => false,
            }))
        }),
        // return function for List-typecheck
        _ => RlType::Func(|arg| {
            Ok(RlType::Bool(match arg[0] {
                RlType::List(..) => true,
                _ => false,
            }))
        }),
    }
}

/**
    Function exists for readability. It returns a function of type RLType::Func that performs Integer-
    Arithmetics. The specific function returned depends on the parameter. (Options: +.-,*,/)

    Arguments: sym - the symbol of the Arithmetic Operation. Determines which function is returned
    Returns: a Function of type RlType::Func performing an Arithmetic Operation
*/
fn integer_arithmetics(sym: &str) -> RlType {
    // check which function is wanted
    match sym {
        // return function for addition (takes a list of integers and returns its sum)
        "+" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x.len() < 1 {return Err(error("+ needs at least 1 parameter"));}
            return Ok(RlType::Int(x.to_vec().iter().sum()));
        }),
        // return function for division(takes a list of integers and returns value of first
        // element divided by sum of rest elements), needs at least 2 args
        "/" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x.len() < 2 {return Err(error("/ needs min 2 params"))}
            let divisor: i64 = x[1..].to_vec().iter().sum();
            if divisor == 0 {
                Err(error("cant divide by zero"))
            } else {
                Ok(RlType::Int(x[0] / divisor))
            }
        }),
        // return function for multiplication (takes a list of integers and returns its product),
        // Needs at least one parameter
        "*" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x.len() < 1 {return Err(error("* needs at least 1 parameter"));}
            return Ok(RlType::Int(x.to_vec().iter().product()));
        }),
        // return function for subtraction (takes a list of integers and returns value of first
        // element minus sum of the rest elements). Needs at least one parameter (with 1 arg arg is negated)
        "-" => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x.len() < 1 {
                return Err(error("- needs at least 1 parameter"));
            } else if x.len() == 1 {
                return Ok(RlType::Int(- x[0]));
            }
            let neg: i64 = x[1..].to_vec().iter().sum();
            Ok(RlType::Int(x[0] - neg))
        }),
        // return function, that checks if first integer is greater than the second one
        ">" => RlType::Func(|a| {
            let x = check_int_vector(a)?;
            if x.len() != 2 {return Err(error("> needs exactly 2 args"));}
            return Ok(RlType::Bool(x[0] > x[1]));
        }),
        // return function, that checks if first integer is smaller than the second one
        _ => RlType::Func(|a: Vec<RlType>| {
            let x = check_int_vector(a)?;
            if x.len() != 2 {return Err(error("> needs exactly 2 args"));}
            return Ok(RlType::Bool(x[0] < x[1]));
        }),
    }
}

/**
    This function is a helper that is used by the Integer-Arithmetics to check if a List contains
    just Integers. It takes a List of expressions, check if every expression is an Integer and
    returns a vector of Integers in Best case. If the given list contains non-integers, return an Error.

    Arguments:  vec - the list of expressions that should be validated to be Integers
    Returns:    a vector containing i64 values extracted from the given list or an Error.
*/
fn check_int_vector(vec: Vec<RlType>) -> Result<Vec<i64>, RlErr> {
    let mut new_vec = Vec::new();
    for element in vec.iter() {
        match element {
            RlType::Int(i) => new_vec.push(i.clone()),
            _ => return Err(error("Wrong kind of arguments!")),
        }
    }
    return Ok(new_vec);
}

/**
    This function returns the Function(RLType::Func) that performs the "car" operation.
    car returns the first element of a given list.

    Returns: The Function that performs the car-operation (Type RLType::Func)
*/
fn car() -> RlType {
    /// Function that performs car operation
    return RlType::Func(|a| match &a[0] {
        // check if given argument is a List
        RlType::List(l) => {
            // if list is empty return Error
            return if l.len() < 1 {
                Err(error("car needs a list of min len 1"))
            } else {
                //println!("{:?}", l);
                // else return first element of the list
                Ok(l.get(0).unwrap().clone())
            }
        }
        // if argument of car is no list, return Error
        //_a => Err(error(&*format!("car expects a list! but got {:?}", _a))),
        // modified to return value back for ROL
        _a => Ok(_a.clone()),
    });
}

/**
    This function returns the Function(RLType::Func) that performs the "cdr" operation.
    cdr returns the second part(the rest) of a given pair.

    Returns: The Function that performs the cdr-operation (Type RLType::Func)
*/
fn cdr() -> RlType {
    /// Function that performs cdr operation
    return RlType::Func(|a| match &a[0] {
        // check if argument given to cdr is a List
        RlType::List(l) => {
            // if list is empty
            return if l.len() < 1 {
                Err(error("cdr needs a list with min len 2!"))
            } else {
                if l.len() == 1 {
                    Ok(RlType::List(vec![]))
                }
                else if l.len() == 2{
                    // else just return the list without the first element
                    Ok(l[1].clone())
                } else {
                    Ok(RlType::List(l[1..].to_vec().clone()))
                }
            }
        }
        // if argument given to car is no list, return an Error
        _ => Err(error("cdr expects a list!")),
    });
}

/**
    This function returns the Function(RLType::Func) that performs the "list" operation.
    List is used to build lists from given elements. Recursively build up pair structure of cons.

    Returns: The Function that performs the list-operation (Type RLType::Func)
*/
fn list(args: Vec<RlType>) -> RlType {
    return if args.len() < 1 {
        RlType::List(vec![])
    } else {
        // if list has two elements, create a List/Pair and returns it
        RlType::List(vec![args[0].clone(), list(args[1..].to_vec().clone())])
    }
}

/**
    This function returns the Function(RLType::Func) that performs the "cons" operation.
    cons is used to prepend an element to a list. If second element is not a list, create
    following structure: (element2 (element2 ()))

    Returns: The Function that performs the cons-operation (Type RLType::Func)
*/
fn cons() -> RlType {
    /// Function that performs the cons operation
    return RlType::Func(|a| {
        // check if given list has 2 elements
        return if a.len() != 2 {
            Err(error("cons needs 2 arguments"))
        } else {
            return match &a[1] {
                // check if second argument is a list -> must be for cons!
                RlType::List(l) => Ok(RlType::List(vec![a[0].clone(), RlType::List(l.clone())])),
                // if its not, pack the second element in a separate pair with tailing empty list
                _ => Ok(RlType::List(vec![a[0].clone(), RlType::List(vec![a[1].clone(), RlType::List(vec![])])])),
            };
        };
    });
}

/**
    This function returns the Function(RLType::Func) that performs the "eq?" operation.
    eq? returns if the two given arguments are equal (equality is defined in types.rs).

    Returns: The Function that performs the eq?-operation (Type RLType::Func)
*/
fn equals() -> RlType {
    /// Function that performs eq?-Operation
    return RlType::Func(|a| {
        // check if eq? has been given exactly 2 arguments
        return if a.len() != 2 {
            // return Error if number of arguments is incorrect
            Err(error("eq? takes exactly 2 args"))
        } else {
            // if 2 arguments given, check equality and return result(of Type RLType::Bool)
            Ok(RlType::Bool(a[0] == a[1]))
        };
    });
}

/*
    This function returns the Function(RlTyype::Func) that perfoms the "%" (modulus) operation.
    % returns the remainder

    Returns: The Function that perfoms the %(-operation (Type RlType::Func)
 */
fn modulo() -> RlType {
    return RlType::Func(|a| {
        // check if parameters are of type RlType::Int
        let x = check_int_vector(a)?;
        // check if % has been given exactly 2 arguments
        return if x.len() != 2 {
            // return Error if number of arguments is incorrect
            Err(error("% takes exxactly 2 args"))
        } else {
            // if 2 arguments given, compute modulus
            Ok(RlType::Int(x[0].clone() % x[1].clone()))
        }
    })
}