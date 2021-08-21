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
pub mod choices;

// load important functionality from submodules
use crate::env::RlEnv;
use types::{RlErr, RlReturn, RlType};
use utils::{read_file_string};

// standard library imports
use structopt::StructOpt;
use crate::choices::{RlChoicesManager, new_choices_manager};
use crate::eval::amb_eval;

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
        // convenience defs for non-det evaluation (for the use of it)
        "(define require (lambda (conditional) (cond (conditional #t) (#t (amb)))))".to_string(),
        "(define try-again (lambda () (amb)))".to_string(),
    ]
}

#[derive(StructOpt)]
struct Cli {
    /// Flag to tell if non-deterministic evaluator should be used
    #[structopt(short = "d", long = "debug", about = "flag that enables debug prints")]
    debug: bool,
    /// The path to the file to read (optional)
    #[structopt(parse(from_os_str), about = "if this argument is given, the interpreter tries to load from the given path")]
    path: Option<std::path::PathBuf>,
}

/**
main() is the entry-point into RLisp, it checks for cmd arguments and if there is an
argument, try to load from file with this name, otherwise start REPL(Read-Eval-Print-Loop)
 */
fn main() {
    let args = Cli::from_args();
    // create a new global environment (stdlib already loaded)
    let env = env::init_global();
    // create new global choices structure (used for non-det pattern) - with placeholder expression
    let choices_manager = new_choices_manager(RlType::Nil, env.clone());
    // execute/evaluate self defined RLisp expressions
    for definition in self_defined_prebuild().iter() {
        rep_wrapper(definition, env.clone(), choices_manager.clone(),false, true);
    }
    // Operate on given arguments
    match &args.path {
        //TODO find a cleaner way to convert path_buf to String
        Some(t) => { load(&t.as_path().display().to_string(), env, choices_manager.clone()) }
        _ => {
            // run normal repl loop
            normal_loop(env.clone(), choices_manager.clone());
        }
    }
}

fn normal_loop(env: RlEnv, choices: RlChoicesManager) {
    loop {
        // use extern crate rustyline, to get userinput
        let mut reader = rustyline::Editor::<()>::new();
        let input = reader.readline("user> ");
        // check if there was a valid input
        match input {
            // There was a valid input
            Ok(line) => {
                // if command == exit, break out of the REPL
                if line == "exit" {
                    break;
                } else if line == "try-again" {
                    // if try-again is executed, try to reevaluate with next choice
                    rep_wrapper(&line, env.clone(), choices.clone(), true, false);
                } else {
                    // if command != exit, call the rep-wrapper with the global environment
                    rep_wrapper(&line, env.clone(), choices.clone(),true, true);
                }
            }
            // There was no valid input -> Give information and repeat the loop
            Err(_) => println!("No input"),
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
            non_det - flag whether normal or non-det evaluator should be used
            success/fail - root-continuations that need to be passed when non_det is true
Returns:    of type RlReturn, so either the result or an (Evaluation)Error
 */
fn EVAL(expression: RlType, env: RlEnv, choices_manager: RlChoicesManager, new_problem: bool) -> RlReturn {
    return amb_eval(expression, env, choices_manager, new_problem);
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
fn rep(to_process: &String, env: RlEnv, choices_manager: RlChoicesManager, new_problem: bool) -> Result<String, RlErr> {
    return Ok(PRINT(EVAL(READ(to_process)?, env, choices_manager, new_problem)?));
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
fn rep_wrapper(to_rep: &String, env: RlEnv, choices_manager: RlChoicesManager ,print_flag: bool, new_problem: bool) {
    // if there was an input
    if to_rep.len() > 0 {
        // if want to wrap non-det program, call amb_rep
        match rep(&to_rep, env, choices_manager, new_problem) {
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
Takes a filename and an environment uses read_file_string to read string from file, then
surrounds this string with a (do ) expression -> This is so that all expressions in the
file are executed even if there are multiple independent ones on multiple lines
(newlines are removed in the parsing process).

Arguments:  filename - name of file to read from
            env - the environment, the loaded expressions should be evaluated in
 */
fn load(filename: &String, env: RlEnv, choices_manager: RlChoicesManager) {
    // load file string and pack into do expression
    let file_string = read_file_string(filename.to_string());
    // process read expression which is before packed into a (do ) expression
    rep_wrapper(&format!("(do {})", file_string), env, choices_manager.clone(),true, true);
}
