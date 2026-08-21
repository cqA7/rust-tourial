use std::ops::RangeBounds;

pub fn push(s: &mut String, append_str: &str) {
    s.push_str(append_str);
    s.push('!');
}

pub fn insert(s: &mut String, index: usize, insert_str: &str) {
    s.insert_str(index, insert_str);
    s.insert(s.len(), '!');
}

pub fn replace(s: String, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn replace_str(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn replacen(s: String, from: &str, to: &str, n: usize) -> String {
    s.replacen(from, to, n)
}

pub fn replacen_str(s: &str, from: &str, to: &str, n: usize) -> String {
    s.replacen(from, to, n)
}

pub fn replace_range<R: RangeBounds<usize>>(s: &mut String, range: R, to: &str) {
    s.replace_range(range, to);
}

pub fn pop(s: &mut String) -> Option<char> {
    s.pop()
}
