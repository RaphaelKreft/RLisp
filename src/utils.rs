use regex::Regex;

pub fn string_is_integer(string: String) -> bool {
    let re = Regex::new(r"^-?[0-9]+$").unwrap();
    return re.is_match(&string);
}
