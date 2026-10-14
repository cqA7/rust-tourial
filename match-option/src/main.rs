/*
因为 `Option`，`Some`，`None` 都包含在 `prelude` 中，因此你可以直接通过名称来使用它们，
而无需以 `Option::Some` 这种形式去使用，
总之，千万不要因为调用路径变短了，就忘记 `Some` 和 `None` 也是 `Option` 底下的枚举成员！
*/
fn main() {
    let five: Option<u8> = Some(5u8);
    if let Option::Some(x) = five {
        println!("The value of x is: {}", x);
    }

    if let Some(x) = five {
        println!("The value of x is: {}", x);
    }

    match five {
        Option::Some(x) => println!("The value of x is: {}", x),
        Option::None => println!("No value found"),
    }

    match five {
        Some(x) => println!("The value of x is: {}", x),
        None => println!("No value found"),
    }

    let six = plus_one(five);
    if let Some(x) = six {
        println!("The value of x is: {}", x);
    }
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
