mod operate;
mod slice;

use operate::{insert, pop, push, replace, replace_range, replace_str, replacen, replacen_str};
use slice::{str_2_string, str_slice, string_2_str};
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

    println!("\n=============push & push_str==============\n");

    let mut s_push_test = String::from("hello");
    push(&mut s_push_test, ", rust");
    println!("测试push方法 -> {}", s_push_test);

    println!("\n=============insert==============\n");

    insert(&mut s_push_test, 5, ", rust");
    println!("测试insert方法 -> {}", s_push_test);

    println!("\n=============replace==============\n");

    let from = "rust";
    let to = "RUST";
    let r = replace(s_push_test, from, to);
    println!("String 测试replace方法 -> {}", r);

    let replace_str_test = "hello, rust";
    let res = replace_str(replace_str_test, from, to);
    println!("&str 测试replace方法 -> {}", res);

    println!("\n=============replacen==============\n");

    let replacen_res = replacen(r, to, from, 1);
    println!("String 测试 replacen 方法 -> {}", replacen_res);

    let replacen_str_test = "hello, rust, rust, rust";
    let replacen_str_res = replacen_str(&replacen_str_test, from, to, 2);
    println!("&str 测试 replacen 方法 -> {}", replacen_str_res);

    println!("\n=============replace_range==============\n");

    let mut string_range_test = String::from("hi, I like rust");
    replace_range(&mut string_range_test, 6..=9, "test");
    println!("String 测试 replace_range 方法 -> {}", string_range_test);

    println!("\n=============pop==============\n");
    let mut string_pop_test = String::from("hi, you");
    if let Some(char_pop) = pop(&mut string_pop_test) {
        println!("pop removed char is {}", char_pop);
    } else {
        println!("no char return");
    }

    match pop(&mut string_pop_test) {
        Some(char_pop) => {
            println!("pop removed char is {}", char_pop);
        }
        None => {
            println!("no char return");
        }
    }

    println!("string_pop_test is {}", string_pop_test);

    println!("\n===========================\n");

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";
    println!("测试链接字符串 -> {}", result)
}
