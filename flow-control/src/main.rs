fn main() {
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num is {}", num);

    let n = 100;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for (i, v) in a.iter().enumerate() {
        println!("the value of a[{}] is: {}", i, v);
    }

    let mut counter = 0;
    while counter <= 5 {
        println!("counter is {}", counter);
        counter += 1;
    }

    let mut n = 0;
    let res = loop {
        n += 1;
        if n == 10 {
            break n * 2;
        }
    };
    println!("The result is {}", res);
}
