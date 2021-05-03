use std::io::{self, Write};
use regex::Regex;

pub fn input(text: &str) -> String {
    let mut input: String = String::new();
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    let len_withoutgarbage = input.trim_end().len();
    input.truncate(len_withoutgarbage);
    return input;
}

pub fn sting_holds_integer(string: String) -> bool {
    let re = Regex::new(r"[+-]?\b[0-9]+\b");
    return re.is_match(string);
}

pub fn string_holds_float(string: String) -> bool {
    let re = Regex::new(r"^[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)$");
    return re.is_match(string);
}