pub fn integer() {
    let i_num: u32 = 100_100;
    let u_num: i32 = -100_100;
    println!("i_num is {},\nu_num is {}", i_num, u_num);

    let a: u8 = 255;
    let added: u8 = 20;
    let b = a.saturating_add(added);
    println!("{} saturating_add {} b is {}", a, added, b);
    let c = a.wrapping_add(added);
    println!("{} wrapping_add {} b is {}", a, added, c);
}

pub fn float() {
    let (a, b, c): (f32, f64, f64) = (0.1, 0.2, 0.3);
    println!("a, b, c is {}, {}, {}", a, b, c);
}

pub fn nan() {
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
}

pub fn bit_operate() {
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // 二进制为00000011
    let b: u8 = 3;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);

    println!("b value is        {:08b}", b);

    println!("(a & b) value is  {:08b}", a & b);

    println!("(a | b) value is  {:08b}", a | b);

    println!("(a ^ b) value is  {:08b}", a ^ b);

    println!("(!b) value is     {:08b}", !b);

    println!("(a << b) value is {:08b}", a << b);

    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);
}

pub fn range() {
    for i in 1..=5 {
        println!("{}", i);
    }
}

pub fn ch() {
    let c = 'c';
    println!("char is {}", c);
}
