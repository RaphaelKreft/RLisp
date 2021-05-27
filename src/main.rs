/*
main.rs: The main file of the RLisp Interpreter.

Use with Commandline-Argument: argument is interpreted as filename and try to load it
Use without Commandline-Argument: Read-Eval-Print Loop is executed , User can input commands line by line
*/

// hook in submodules (Defines module-structure)
mod env;
mod eval;
mod printer;
mod reader;
mod stdlib;
pub mod types;
pub mod utils;

// load important functionality from submodules
use crate::env::RlEnv;
use eval::eval;
use std::fs;
use types::{RlErr, RlReturn, RlType};

#[macro_use]
extern crate lazy_static;

extern crate rustyline;

/**
    This function returns a vector of RLisp expressions, that define functions in RLisp itself.
    Here Users can add definitions or commands that should be executed every time executing RLisp.

    Returns: Vector of Strings that should contain RLisp expressions
*/
fn self_defined_prebuild() -> Vec<String> {
    vec![
        "(define caar (lambda (x) (car (car x))))".to_string(),
        "(define cadr (lambda (x) (car (cdr x))))".to_string(),
        "(define cddr (lambda (x) (cdr (cdr x))))".to_string(),
        "(define caddr (lambda (x) (cadr (cdr x))))".to_string(),
        "(define cadar (lambda (x) (cadr (car x))))".to_string(),
        "(define caddar (lambda (x) (caddr (car x))))".to_string(),
        "(define cddar (lambda (x) (cddr (car x))))".to_string(),
        "(define caadar (lambda (x) (caar (cdr (car x)))))".to_string(),
    ]
}

/**
    main() is the entry-point into RLisp, it checks for cmd arguments and if there is an
    argument, try to load from file with this name, otherwise start REPL(Read-Eval-Print-Loop)
*/
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // create a new global environment (stdlib already loaded)
    let env = env::init_global();
    // execute/evaluate self defined RLisp expressions
    for definition in self_defined_prebuild().iter() {
        rep_wrapper(definition, env.clone(), false);
    }

    if args.len() == 2 {
        // if argument was given open and operate on file input
        load(&args[1], env.clone());
    } else {
        // else operate in cmd mode -> REPL
        let mut rl = rustyline::Editor::<()>::new();
        loop {
            // use extern crate rustyline, to get userinput
            let input = rl.readline("user> ");
            // check if there was a valid input
            match input {
                // There was a valid input
                Ok(line) => {
                    // if command == exit, break out of the REPL
                    if line == "exit" {
                        break;
                    }
                    // if command != exit, call the rep-wrapper with the global environment
                    rep_wrapper(&line, env.clone(), true);
                }
                // There was no valid input -> Give information and repeat the loop
                Err(_) => println!("No input"),
            }
        }
    }
}

/**
    Calls reader to convert a string to an AST(Abstract Syntax tree). That is a language specific
    Data-structure that the Evaluator can work with internally.

    Arguments:  plain_input - a String that should be parsed into an AST
    Returns:    Of type RLReturn, so either a valid AST or an (Parsing)Error
*/
fn READ(plain_input: &String) -> RlReturn {
    return reader::read_str(plain_input.to_string());
}

/**
    Takes an AST and an Environment, calls eval::eval(The Interface to the Evaluator-Part of the Interpreter)
    and returns the result of the Evaluator.

    Arguments:  expression - the AST(type RLType) that should be evaluated
                env - the environment that the evaluator should work with
    Returns:    of type RlReturn, so either the result or an (Evaluation)Error
*/
fn EVAL(expression: RlType, env: RlEnv) -> RlReturn {
    return eval(expression, env);
}

/**
    Takes an AST and calls printer::print_str to turn AST into String, then returns this String

    Arguments:  text - the AST that should be converted to a string
    Returns:    the String to the given text
*/
fn PRINT(text: RlType) -> String {
    return printer::print_str(text);
}

/**
   read-eval-print: Chain calls READ-EVAL-PRINT

   Arguments:   to_process - an input string that should be interpreted
                env - the environment the expression/input string is evaluated in
   Returns:     Error? -> object with type RLError is returned, otherwise the result sting
*/
fn rep(to_process: &String, env: RlEnv) -> Result<String, RlErr> {
    return Ok(PRINT(EVAL(READ(to_process)?, env)?));
}

/**
    Interface to rep:
    It calls rep with the string and the environment, print out results or errors nicely if print
    flag is set.

    Arguments:  to_rep - the string to process
                env - the environment, that the Evaluator works with
                print_flag - determines whether the errors or results are printed out

    Returns:    -
*/
fn rep_wrapper(to_rep: &String, env: RlEnv, print_flag: bool) {
    // if there was an input
    if to_rep.len() > 0 {
        // call rep to process input, then check if there was
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

/**
    Takes a filename and reads the content as String, if an error occurs, NIL is returned

    Arguments:  filename - the name of the file to read as String
    Returns:    file-content as String
*/
fn read_file_string(filename: String) -> String {
    return fs::read_to_string(filename).expect("#nil");
}

/**
    Takes a filename and an environment uses read_file_string to read string from file, then
    surrounds this string with a (do ) expression -> This is so that all expressions in the
    file are executed even if there are multiple independent ones on multiple lines
    (newlines are removed in the parsing process).

    Arguments:  filename - name of file to read from
                env - the environment, the loaded expressions should be evaluated in
*/
fn load(filename: &String, env: RlEnv) {
    // load file string and pack into do expression
    let file_string = read_file_string(filename.to_string());
    let to_execute = format!("(do {})", file_string);
    // execute prepared expression
    rep_wrapper(&to_execute, env, true)
}
