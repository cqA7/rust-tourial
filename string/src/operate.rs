pub fn push(s: &mut String, append_str: &str) {
    s.push_str(append_str);
    s.push('!');
}

pub fn insert(s: &mut String, index: usize, insert_str: &str) {
    s.insert_str(index, insert_str);
    s.insert(s.len(), '!');
}
