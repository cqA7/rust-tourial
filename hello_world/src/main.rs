fn _greet_world() {
    let penguin_data = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Invalid,data
        ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn _variable() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);
    b = true;
    println!("after change a = {}, b = {}", a, b);
    assert_eq!(a, b);
    const MAX_COUNT: u32 = 1000;
    println!("MAX_COUNT: {}", MAX_COUNT);
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);
}

fn _char_bool() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", size_of_val(&x));
}

// 函数就是一个表达式
// Rust 的函数体是由一系列语句组成，最后由一个表达式来返回值，例如：
// 表达式会进行求值，语句不会
fn statement_expression(x: i32) -> i32 {
    let x = x + 1;
    let y = if x % 2 == 1 { "odd" } else { "even" };
    println!("y is {}", y);
    x
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    // greet_world();
    // variable();
    // char_bool();
    let y = statement_expression(10);
    println!("y = {}", y);
    let sum = add(10, 4);
    println!("sum is {}", sum);
}
