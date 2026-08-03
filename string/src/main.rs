fn main() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    println!("{}", string_remove);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("测试链接字符串 -> {}", result)
}
