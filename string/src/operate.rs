pub fn push(s: &mut String, append_str: &str) {
    s.push_str(append_str);
    s.push('!');
}

pub fn insert(s: &mut String, index: usize, insert_str: &str) {
    s.insert_str(index, insert_str);
    s.insert(s.len(), '!');
}

pub fn replace(s: &mut String, from: &str, to: &str) {
    *s = s.replace(from, to);
}

pub fn replace_str(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}
