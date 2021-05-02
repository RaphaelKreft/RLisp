use std::io::{self, Write};

pub fn input(text: &str) -> String {
    let mut input: String = String::new();
    print!("{}", text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).ok();
    let len_withoutgarbage = input.trim_end().len();
    input.truncate(len_withoutgarbage);
    return input;
}