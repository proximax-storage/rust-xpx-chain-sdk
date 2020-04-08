use regex::Regex;

pub fn is_hex(input: &str) -> bool {
    if input == "" {
        return false;
    }

    let re = Regex::new(r"^[a-fA-F0-9]+$").unwrap();

    re.is_match(input)
}
