/*
reader.rs:  Holds the parser for S-Expressions, that is used to build up the sytaxtree
            Datastructure from input. The resulting Datastrucure is later used by eval.
*/

use regex::Regex;
use super::types::{Atom, Sexpression, get_value_list};


// This is the only function visible and the Interface of the whole
// reader functionality
pub fn read_str(line: &String) -> Sexpression {
    let mut tmpRead: Reader = Reader::new(tokenize(line));
    let plist = Sexpression::List(Vec::new());
    return read_from_tokens(&mut tmpRead, &mut plist);
}

// Takes a Reader and creates an Sexpression with the help of it
fn read_from_tokens(reader: &mut Reader, plist: &mut Sexpression) -> Sexpression {
    if reader.peek() == "(" {
        return read_list(reader, plist);
    } else {
        return read_atom(reader);
    }
}

fn read_list(reader: &mut Reader, plist: &mut Sexpression) {
    let mylist = Sexpression::List(Vec::new());
    loop {
        if reader.next() == ")" {
            get_value_list(plist).push(mylist);
            break;
        }
        get_value_list(mylist).push(read_from_tokens(reader, plist));
    }
}

fn read_atom(reader: &mut Reader, plist: &mut Sexpression) {
    let atom = reader.next();
    if get_value_list(plist).is_empty() {
        *plist = Sexpression::Atom(Atom::new(&atom.to_string()));
    } else {
        get_value_list(plist).push(Sexpression::Atom(Atom::new(&atom.to_string())));
    }
}


// extract all valid tokens out of a String
fn tokenize(input: &String) -> Vec<String>{
    let vec: Vec<String> = Vec::new();
    // extract all tokens from input string and store in vector
    let re = Regex::new(r"").unwrap();
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

