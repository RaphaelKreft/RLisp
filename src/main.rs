/*
main.rs: The main file of the RLisp Interpreter.
*/

mod printer;
mod reader;
pub mod utils;
pub mod types;
//mod env;
//mod eval;

use std::fs::File;
use std::io::{self, BufRead};
use types::{RlType};
use crate::types::{RlReturn, RlErr, Atom, AtomType};

#[macro_use]
extern crate lazy_static;

extern crate rustyline;
use rustyline::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // if argument was given open and operate on file input
        load(&args[1]);
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
                    rep_wrapper(&line);},
                Err(_) => println!("No input"),
            }
        }
    }
}

// Wrapper of REPL: calls reader to convert a string to an AST the Evaluator can work with
fn READ(plain_input: &String) -> RlReturn {
    return reader::read_str(plain_input.to_string());
}

fn EVAL(expression: RlType) -> RlReturn {
    return Ok(expression);
}

fn PRINT(text: RlType) -> String {
    return printer::print_str(text);
}

// Chain calls READ-EVAL-PRINT
fn rep(to_process: &String) -> Result<String, RlErr> {
    return Ok(PRINT(EVAL(READ(to_process)?)?));
}

// Interface to rep, to handle and print out results or errors nicely
fn rep_wrapper(to_rep: &String) {
    if to_rep.len() > 0 {
        match rep(&to_rep) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Exception! {}: ", err),
        }
    }
}

// For now parse an load are in the main -> move to env later?
fn parse(toparse: &String) -> Result<String, RlErr> {
    return Ok(PRINT(READ(toparse)?));
}

fn load(filename: &String) {
    if let Ok(lines) = read_lines(filename) {
        // Iterate over lines and check if they are ok(Or EOF)
        for line in lines {
            if let Ok(input) = line {
                if input.starts_with(";") {
                    continue;
                }
                rep_wrapper(&input);
            }
        }
    } else {
        println!{"there was a problem with your file!"};
    }
}

// Read lines from a file TODO: Add preprocessing for multiline expressions
fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use crate::types::{RlType, Atom, AtomType};

    #[test]
    fn tokenize_test() {
        assert_eq!(vec!["(", "define", "x", "5", ")"], super::reader::tokenize("(define x 5)"));
        assert_eq!(vec!["5"], super::reader::tokenize("   5   "));
        assert_eq!(vec!["(", "parse", "'", "(", "+", "2", "3", ")", "'", ")"], super::reader::tokenize("(parse '(+ 2 3)')"));
    }

    #[test]
    fn parse_to_ast_test() {
        let ast = super::reader::read_str("5".to_string()).ok();
        let ast2 = super::reader::read_str("(+)".to_string()).ok();
        let ast3 = super::reader::read_str("(+ 2 (*2 3))".to_string()).ok();
        assert_eq!(ast, RlType::Atom(Atom{value: "5".to_string(), typ: AtomType::Int}));
        assert_eq!(ast2, RlType::List(vec![RlType::Atom(Atom{value: "+".to_string(), typ: AtomType.Symbol})]));
        assert_eq!(ast3, RlType::List(vec![RlType::Atom(Atom{value: "+".to_string(), typ: AtomType.Symbol}),
                                           RlType::Atom(Atom{value: "2".to_string(), typ: AtomType.Int}),
                                           RlType::List(vec![RlType::Atom(Atom{value: "*".to_string(), typ: AtomType.Int}),
                                           RlType::Atom(Atom{value: "2".to_string(), typ: AtomType.Int}),
                                                             RlType::Atom(Atom{value: "3".to_string(), typ: AtomType.Int})])]));
    }

    #[test]
    fn read_print_test() {

    }
}
