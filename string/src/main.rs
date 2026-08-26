mod operate;
mod slice;

use operate::{
    clear, insert, pop, push, remove, replace, replace_range, replace_str, replacen, replacen_str,
    truncate,
};
use slice::{str_2_string, str_slice, string_2_str};
use std::{mem::size_of_val, ops::Add};

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

    println!("\n=============remove==============\n");

    let mut string_remove_test = String::from("测试 remove 方法");
    let ch = remove(&mut string_remove_test, 0);
    println!("removed char is {}", ch);

    println!("\n=============truncate==============\n");

    let mut string_truncate_test = String::from("测试 truncate 方法");
    truncate(&mut string_truncate_test, 6);
    println!("测试 truncate 方法 -> {}", string_truncate_test);

    println!("\n=============clear==============\n");

    let mut string_clear_test = String::from("测试 clear 方法");
    clear(&mut string_clear_test);
    println!("测试 clear 方法 -> {}", string_clear_test);

    println!("\n=============连接(Concatenate)==============\n");

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    // 下面两种方式：通过 + 进行链接 和 调用 add 进行链接是等价的
    // add 方法的定义是：fn add(self, s: &str) -> String
    // let result = string_append + &string_rust;
    let result = string_append.add(&string_rust);
    let mut result = result + "!";
    result += "!!!";
    println!("测试链接字符串 -> {}", result);

    println!("\n=============连接(Concatenate)-format==============\n");
    let s1 = "hello";
    let s2 = String::from("rust");
    let format_string = format!("{} {}", s1, s2);
    println!("s1 is {}, s2 is {}", s1, s2);
    println!("format string is {}", format_string);

    println!("\n=============字符串转义==============\n");

    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    println!("\n=============遍历 UTF-8 字符串-char==============\n");

    let iter_string = String::from("中国人");
    for c in iter_string.chars() {
        println!("{}", c);
    }

    let iter_str = "中国人";
    for c in iter_str.chars() {
        println!("{}", c);
    }

    println!("\n=============遍历 UTF-8 字符串-byte==============\n");

    let iter_string = String::from("中国人");
    for c in iter_string.bytes() {
        println!("{}", c);
    }

    let iter_str = "中国人";
    for c in iter_str.bytes() {
        println!("{}", c);
    }
}
