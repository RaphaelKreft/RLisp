/*
main.rs: The main file of the RLisp Interpreter.
*/

mod env;
mod eval;
mod printer;
mod reader;
mod stdlib;
pub mod types;
pub mod utils;

use crate::env::RlEnv;
use eval::eval;
use std::fs;
use types::{RlErr, RlReturn, RlType};

#[macro_use]
extern crate lazy_static;

extern crate rustyline;

/// This function returns a vector of Rlisp expressions, that define functions in RLisp itself
fn self_defined_prebuild() -> Vec<String> {
    vec![
        "(define caar (lambda (x) (car (car x))))".to_string(),
        "(define cadr (lambda (x) (car (cdr x))))".to_string(),
        "(define caddr (lambda (x) (cadr (cdr x))))".to_string(),
        "(define cadar (lambda (x) (cadr (car x))))".to_string(),
        "(define caddar (lambda (x) (caddr (car x))))".to_string(),
        "(load 'jmc_adapted.txt')".to_string(),
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let env = env::init_global();

    for sdef in self_defined_prebuild().iter() {
        rep_wrapper(sdef, env.clone(), false);
    }

    if args.len() == 2 {
        // if argument was given open and operate on file input
        load(&args[1], env.clone());
    } else {
        // else operate in cmd mode
        let mut rl = rustyline::Editor::<()>::new();
        loop {
            // Input string
            let input = rl.readline("user> ");
            match input {
                Ok(line) => {
                    if line == "exit" || line.starts_with(";") {
                        break;
                    }
                    rep_wrapper(&line, env.clone(), true);
                }
                Err(_) => println!("No input"),
            }
        }
    }
}

/// calls reader to convert a string to an AST the Evaluator can work with
fn READ(plain_input: &String) -> RlReturn {
    return reader::read_str(plain_input.to_string());
}
/// takes an AST and an Environment, calls eval and returns its value
fn EVAL(expression: RlType, env: RlEnv) -> RlReturn {
    return eval(expression, env);
}
/// Takes an AST and calls printer::print_str to turn AST into String, then returns this String
fn PRINT(text: RlType) -> String {
    return printer::print_str(text);
}

/// read-eval-print: Chain calls READ-EVAL-PRINT
fn rep(to_process: &String, env: RlEnv) -> Result<String, RlErr> {
    return Ok(PRINT(EVAL(READ(to_process)?, env)?));
}

/** Interface to rep: takes a string that should be interpreted, an environment and a print_flag.
    It calls rep with the string and the environment, print out results or errors nicely if print
    flag is set. */
fn rep_wrapper(to_rep: &String, env: RlEnv, print_flag: bool) {
    if to_rep.len() > 0 {
        match rep(&to_rep, env) {
            Ok(res) => {
                if print_flag {
                    println!("{}", res)
                }
            }
            Err(err) => {
                if print_flag {
                    println!("Exception! {}: ", err)
                }
            }
        }
    }
}
/// takes a Filename as a string and read the content as String, if an error occurs, NIL is returned
fn read_file_string(filename: String) -> String {
    return fs::read_to_string(filename).expect("#nil");
}

/** takes a filename and an environment uses read_file_string to read string from file, then
    surrounds this string with a (do ) expression -> This is so that all expressions in the
    file are executed even if there are multiple independent ones on multiple lines
    (newlines are removed in reading process) */
fn load(filename: &String, env: RlEnv) {
    let rstring = read_file_string(filename.to_string());
    let to_execute = format!("(do {})", rstring);
    rep_wrapper(&to_execute, env, true)
}
