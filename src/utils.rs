/*
utils.rs: contains utility functions
 */

// import needed Rust-functionality
use regex::Regex;
use std::fs;

/**
    Takes a String and checks whether the string represents an Integer using regex.

    Arguments:  string - the string to check if it represents an Integer
    Returns:    bool - whether the given string represents an Integer or not
*/
pub fn string_is_integer(string: String) -> bool {
    // create new Regular expression that matches Integers
    let re = Regex::new(r"^-?[0-9]+$").unwrap();
    // test if given string is a match and return according bool
    return re.is_match(&string);
}

/**
Takes a filename and reads the content as String, if an error occurs, program exist with exitcode 1

Arguments:  filename - the name of the file to read as String
Returns:    file-content as String
 */
pub fn read_file_string(filename: String) -> String {
    match fs::read_to_string(filename.clone()) {
        Ok(content) => {return content;},
        Err(_) => {
            println!("Could not read file {}, probably it wasn't found!", filename);
            std::process::exit(0x001);
        }
    }
}

