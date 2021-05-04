/*
reader.rs:  Holds the parser for S-Expressions, that is used to build up the Sytaxtree
            Data_structure from input. The resulting Data_strucure is later used by eval.
*/

use regex::{Regex};


use super::types::{Atom, RlType, error};
use crate::types::{RlReturn, RlErr};
use crate::types::RlErr::ErrString;


// This is the only function visible and the Interface of the whole
// reader functionality
pub fn read_str(line: String) -> RlReturn {
    //println!("Got line: {}", line);
    let tokens = tokenize(&line);
    if tokens.is_empty() {
        return error("No valid tokens found");
    }
    //println!("Got tokens: {:?}", tokens);
    return read_from_tokens(&mut Reader::new(tokens));
}

// Takes a Reader and creates an Sexpression with the help of it
fn read_from_tokens(reader: &mut Reader) -> RlReturn {
    let peeked_token = reader.peek()?;
    match &peeked_token[..] {
        "(" => read_list(reader),
        _ => read_atom(reader),
    }
}

fn read_list(reader: &mut Reader) -> RlReturn {
    let mut my_list: Vec<RlType> = vec![];
    reader.next()?; // skip opening bracket
    loop {
        // return error if suddenly EOF occurs
        let token = reader.peek()?;
        if token == ")" {
            reader.next()?;
            break;
        }
        my_list.push(read_from_tokens(reader)?);
    }
    return Ok(RlType::List(my_list));
}

fn read_atom(reader: &mut Reader) -> RlReturn {
    let atom = reader.next()?;
    return Ok(RlType::Atom(Atom::new(&atom.to_string())));
}


// extract all valid tokens out of a String
pub fn tokenize(str: &str) -> Vec<String> {
    // global immutable initialized at runtime
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    for cap in RE.captures_iter(str) {
        // if capture starts with ; its a comment
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }
    return res;
}

// Reader struct is a helper struct to operate on tokenized inputs.
// It holds a list of tokens and has the functions peek, next
struct Reader {
    position: usize,
    tokens: Vec<String>
}

impl Reader {
    fn new(tokens: Vec<String>) -> Reader {
        return Reader {position: 0, tokens};
    }
    // Returns token at current position
    fn peek(&self) -> Result<String, RlErr> {
        return Ok(self.tokens.get(self.position)
                             .ok_or(ErrString("nothing to peek".to_string()))?.to_string());
    }
    // Returns token at current position and moves to next position
    fn next(&mut self) -> Result<String, RlErr> {
        self.position += 1;
        return Ok(self.tokens.get(self.position - 1)
                             .ok_or(ErrString("nothing to peek".to_string()))?.to_string());
    }
}

