/*
main.rs: The main file of the RLisp Interpreter.
*/

mod reader;
mod utils;

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // if argument was given open and operate on file input
        // Check if file exists or has problems
        if let Ok(lines) = read_lines(&args[1]) {
            // Iterate over lines and check if they are ok(Or EOF)
            for line in lines {
                if let Ok(input) = line {
                    println!("{}", rep(&input));
                }
            }
        } else {
            println!{"there was a problem with your file!"}
        }
    } else {
        // else operate in cmd mode
        let mut input = String::new();
        while input != "exit" {
            // Input string
            input = utils::input("user> ");
            // run rep
            println!("{}", rep(&input));
        }
    }
}

fn READ(plain_input: &String) -> &String {
    return plain_input;
}

fn EVAL(expression: &String) -> &String {
    return expression;
}

fn PRINT(text: &String) -> &String {
    return text;
}

fn rep(toprocess: &String) -> &String{
    return PRINT(EVAL(READ(toprocess))); 
}

fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
