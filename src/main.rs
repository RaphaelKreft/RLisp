/*
main.rs: The main file of the RLisp Interpreter.
*/

mod printer;
mod reader;
pub mod utils;
pub mod types;
mod env;
mod eval;
mod stdlib;

use std::fs;
use types::{RlType, RlReturn, RlErr};
use eval::eval;
use crate::env::RlEnv;

#[macro_use]
extern crate lazy_static;

extern crate rustyline;

fn self_defined_prebuild() -> Vec<String> {
    vec![
        "(define caar (lambda (x) (car (car x))))".to_string(),
        "(define cadr (lambda (x) (car (cdr x))))".to_string(),
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let env = env::init_global();

    for sdef in self_defined_prebuild().iter() {
        rep_wrapper(sdef, env.clone());
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
                    if line == "exit" || line.starts_with(";"){
                        break;
                    }
                    rep_wrapper(&line, env.clone());},
                Err(_) => println!("No input"),
            }
        }
    }
}

// Wrapper of REPL: calls reader to convert a string to an AST the Evaluator can work with
fn READ(plain_input: &String) -> RlReturn {
    return reader::read_str(plain_input.to_string());
}

fn EVAL(expression: RlType, env: RlEnv) -> RlReturn {
    return eval(expression,  env);
}

fn PRINT(text: RlType) -> String {
    return printer::print_str(text);
}

// Chain calls READ-EVAL-PRINT
fn rep(to_process: &String, env: RlEnv) -> Result<String, RlErr> {
    return Ok(PRINT(EVAL(READ(to_process)?, env)?));
}

// Interface to rep, to handle and print out results or errors nicely
fn rep_wrapper(to_rep: &String, env: RlEnv) {
    if to_rep.len() > 0 {
        match rep(&to_rep, env) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Exception! {}: ", err),
        }
    }
}

fn read_file_string(filename: String) -> String {
    return fs::read_to_string(filename).expect("#nil");
}

fn load(filename: &String, env: RlEnv) {
    let rstring = read_file_string(filename.to_string());
    println!("{:?}", rstring);
    let to_execute = format!("(do {})", rstring);
    rep_wrapper(&to_execute, env)
}
