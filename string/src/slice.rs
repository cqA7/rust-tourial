pub fn str_slice(s: &String) -> &str {
    &s[0..3]
}

pub fn str_2_string(s: &str) -> String {
    String::from(s)
}

pub fn string_2_str(s: &String) -> &str {
    &s[..]
}
