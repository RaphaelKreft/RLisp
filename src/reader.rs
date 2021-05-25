/*
reader.rs:  Holds the parser for S-Expressions, that is used to build up the Syntax-Tree(RLType)
            Data_structure from a given input(String).
            The resulting Data_structure is later used by eval.
*/

// load needed Rust-Functionality
use regex::Regex;

// load needed sibling-modules
use super::types::{error, RlErr, RlReturn, RlType};
use super::utils;
use crate::types::RlErr::ErrString;

/**
    This is the only function visible and the Interface of the whole
    reader functionality. It takes a String and returns the according SyntaxTree(RLType).
    If there are errors while parsing, an RLError is returned.

    Arguments:  string - the string that will be parsed into an RLType.
    Returns:    ParserError -> return RLError, resulting Syntax Tree otherwise
*/
pub fn read_str(string: String) -> RlReturn {
    // first tokenize the string -> list with valid RLisp tokens
    let tokens = tokenize(&string);
    // if no tokens found, return error
    if tokens.is_empty() {
        return Err(error("No valid tokens found"));
    }
    //println!("Got tokens: {:?}", tokens);
    // use read_from_tokens to build up Syntax-tree from tokens
    return read_from_tokens(&mut Reader::new(tokens));
}

/**
    Takes a Reader-Instance and creates a Syntax-Tree(RLType) with the help of it.

    Arguments:  reader - Reader structure that holds token-list as well as position in this List
    Returns:    RLError when a parsing error occurs, resulting Syntax-tree otherwise
*/
fn read_from_tokens(reader: &mut Reader) -> RlReturn {
    // check what the current token is
    let peeked_token = reader.peek()?;
    match &peeked_token[..] {
        // if token is opening bracket, we expect a list and call read_list()
        "(" => read_list(reader),
        // if token is double quote, we expect a String and call read_string()
        "[" => read_string(reader),
        // if token is simple quote ', read a quoted expression
        "'" => read_quote(reader),
        // else we expect Atomic Elements
        _ => read_atom(reader),
    }
}

/**
    Takes a Reader-Instance and collects a string from the token list until a closing ] is found.

    Arguments:  reader - the Reader Instance that holds the token list and current position in token-list
    Returns:    A RLType::String, holding the read string, If an Error occurs -> RLError
*/
fn read_string(reader: &mut Reader) -> RlReturn {
    // create new empty String instance
    let mut string_tokens: Vec<String> = vec![];
    // skip opening quote
    reader.next()?;
    // loop until closing quote is found
    loop {
        // return error if suddenly EOF occurs
        let token = reader.peek()?;
        if token == "]" {
            // if closing quote occurs, stop adding tokens to string
            reader.next()?;
            break;
        }
        // add token to string
        string_tokens.push(reader.next()?);
    }
    return Ok(RlType::String(string_tokens.join(" ")));
}

/**
    Takes a Reader-Instance and collects a quoted expression from the token list. Therefore just
    the first expression after the quoting symbol ' is read and wrapped by a "quote" special form.

    Arguments:  reader - the Reader Instance that holds the token list and current position in token-list
    Returns:    resulting AST of form (quote {following expression}), If an Error occurs -> RLError
*/
fn read_quote(reader: &mut Reader) -> RlReturn {
    // skip quoting symbol
    reader.next()?;
    // pack following expression in a quote operation -> (quote {following_expression})
    return Ok(RlType::List(vec![RlType::Symbol("quote".to_string()), read_from_tokens(reader)?]));
}

/**
    Takes a Reader-Instance and collects a list from the token list until a closing bracket is found.
    Internally uses recursive calls to resolve the elements of the list.

    Arguments:  reader - the Reader Instance that holds the token list and current position in token-list
    Returns:    A RLType::List, holding the read list, If an Error occurs -> RLError
*/
fn read_list(reader: &mut Reader) -> RlReturn {
    // create vector that will contain the list-elements
    let mut my_list: Vec<RlType> = vec![];
    // skip opening bracket
    reader.next()?;
    // loop until closing bracket occurs
    loop {
        // return error if suddenly EOF occurs
        let token = reader.peek()?;
        if token == ")" {
            // If closing bracket occurs, stop adding list-element to the vector
            reader.next()?;
            break;
        }
        // push list element. List element is return value of read_from_tokens() this is needed to
        // capture nested lists/expressions.
        my_list.push(read_from_tokens(reader)?);
    }
    return Ok(RlType::List(my_list));
}

/**
    Takes a Reader-Instance and collects an atomic value.

    Arguments:  reader - the Reader Instance that holds the token list and current position in token-list
    Returns:    An atomic RLType, depending on the token, If an Error occurs -> RLError
*/
fn read_atom(reader: &mut Reader) -> RlReturn {
    // get next token which is an atom candidate
    let atom = reader.next()?.clone();
    // check if token represents an Integer
    return if utils::string_is_integer(atom.clone()) {
        Ok(RlType::Int(atom.parse().unwrap()))
    } else if atom == "#t" {
        // token #t represents true
        Ok(RlType::Bool(true))
    } else if atom == "#f" {
        // token #f represents false
        Ok(RlType::Bool(false))
    } else if atom == "#nil" {
        // token #nil represents NIL
        Ok(RlType::Nil)
    } else {
        // else interpret token as symbol
        Ok(RlType::Symbol(atom.to_string()))
    };
}

/**
    Takes a string and extracts all valid tokens, that are important for RLisp.

    Arguments:  str - the string to tokenize
    Returns:    a list of strings(tokens)
*/
pub fn tokenize(str: &str) -> Vec<String> {
    // global immutable initialized at runtime -> Initialize regex to capture tokens(taken from MAL)
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];
    // iterate over regex captures(=tokens)
    for cap in RE.captures_iter(str) {
        // if capture starts with ; its a comment, so dont add it
        if cap[1].starts_with(";") {
            continue;
        }
        // else add this token to the list
        res.push(String::from(&cap[1]));
    }
    return res;
}

/**
    Reader struct is a helper struct to operate on tokenized inputs.
    It holds a list of tokens(Strings) and the current position in the token list (needed so that
    reader functions can work without passing list-positions).
*/
struct Reader {
    position: usize,
    tokens: Vec<String>,
}

impl Reader {
    /**
        Static function that is used to crete a Reader Instance -> Constructor

        Arguments:  tokens -  list of tokens to add to the reader
        Returns:    a new reader with position 0 and the given tokens
    */
    fn new(tokens: Vec<String>) -> Reader {
        // initialize with position 0
        return Reader {
            position: 0,
            tokens,
        };
    }

    /**
        Returns token at current position

        Returns:    if there is nothing to peek RLError, else the token at current position of
                    the reader instance(self)
    */
    fn peek(&self) -> Result<String, RlErr> {
        return Ok(self
            .tokens
            .get(self.position)
            .ok_or(ErrString("nothing to peek".to_string()))?
            .to_string());
    }

    /**
        Returns token at current position and moves to next position.

        Returns:    If there is no further token, return RlError, else the token at current position
                    of the reader instance(self)
    */
    fn next(&mut self) -> Result<String, RlErr> {
        // increment position counter (move to next token)
        self.position += 1;
        return Ok(self
            .tokens
            .get(self.position - 1)
            .ok_or(ErrString("nothing to peek".to_string()))?
            .to_string());
    }
}
