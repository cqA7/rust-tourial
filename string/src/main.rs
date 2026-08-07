mod operate;
mod slice;

use operate::*;
use slice::*;
use std::mem::size_of_val;

fn main() {
    let string = String::from("hello, rust");
    let str = str_slice(&string);
    println!("str is {}", str);

    let str1 = "hello, rust";
    let string1 = str_2_string(str1);
    println!("string1 is {}", string1);

    let str2 = string_2_str(&string);
    println!("str2 is {}", str2);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        size_of_val(string_remove.as_str())
    );
    string_remove.remove(0);
    println!("{}", string_remove);

    let mut s_push_test = String::from("hello");
    push(&mut s_push_test, ", rust");
    println!("测试push方法 -> {}", s_push_test);

    insert(&mut s_push_test, 5, ", rust");
    println!("测试insert方法 -> {}", s_push_test);

    let from = "rust";
    let to = "RUST";
    replace(&mut s_push_test, from, to);
    println!("String 测试replace方法 -> {}", s_push_test);

    let replace_str_test = "hello, rust";
    let res = replace_str(replace_str_test, from, to);
    println!("&str 测试replace方法 -> {}", res);

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("测试链接字符串 -> {}", result)
}
