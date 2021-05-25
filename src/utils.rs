/*
utils.rs: contains utility functions
 */

// import needed Rust-functionality
use regex::Regex;

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
