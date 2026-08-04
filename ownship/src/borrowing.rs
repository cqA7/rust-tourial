pub fn borrow() {
    let x = 5;
    let y = &x; // y 是 x 的引用
    println!("y: {}", y);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // y 的作用域结束
    // x 的作用域结束
}

pub fn immutable_borrow() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递 s1 的引用给函数 calculate_length
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn change(s: &mut String) {
    s.push_str(", world");
}
