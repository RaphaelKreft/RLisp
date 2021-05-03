/*
reader.rs:  Holds the parser for S-Expressions, that is used to build up the sytaxtree
            Datastructure from input. The resulting Datastrucure is later used by eval.
*/

use regex::Regex;
use super::types::{Atom, Sexpression};


// This is the only function visible and the Interface of the whole
// reader functionality
pub fn read_str(line: &String) -> Sexpression {
    let tmpRead: Reader = Reader::new(tokenize(line));
    return read_form(tmpRead);
}

// Takes a Reader and creates an Sexpression with the help of it
fn read_form(reader: &Reader) -> Sexpression {
    if reader.peek() == "("{
        return read_list(reader);
    } else {
        return read_atom(reader);
    }
}

fn read_list(reader: &Reader) -> Sexpression{
    let mylist: Sexpression::List;
    loop {
        if reader.peek() == ")" {
            return mylist;
        }
        mylist.push(read_form(reader));
    }
}

fn read_atom(reader: &Reader) -> Sexpression{
    let atom = reader.next();
    return Atom(atom); 
}


// extract all valid tokens out of a String
fn tokenize(input: &String) -> Vec<String>{
    let vec: Vec<String> = Vec::new();
    // extract all tokens from input string and store in vector
    let re = Regex::new(r"");
    return vec;
}

// Reader struct is a helper struct to operate on tokenized inputs.
// It holds a list of tokens and has the functions peek, next
struct Reader {
    position: i64,
    tokens: Vec<String>
}

impl Reader {
    fn new(tokens: Vec<String>) -> Reader {
        return Reader {position: 0, tokens: tokens};
    }
    // Returns token at current position
    fn peek(&self) -> &String {
        return &self.tokens[self.position as usize];
    }
    // Returns token at current position and moves to next position
    fn next(&mut self) -> &String {
        self.position += 1;
        return &self.tokens[(self.position - 1) as usize];
    }
}

