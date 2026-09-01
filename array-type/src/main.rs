mod example;

use example::example;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let a1: [i32; 5] = [2, 3, 4, 5, 6];
    let a2 = [3; 5];

    println!("array a is {:?}", a);
    println!("array a1 is {:?}", a1);
    println!("array a2 is {:?}", a2);

    println!("array a first item is {}", a[0]);

    let arr_string: [String; 8] = std::array::from_fn(|i| format!("rust is good{}", i));
    println!("arr_string is {:#?}", arr_string);

    let a3: [i32; 5] = [9, 8, 7, 6, 5];
    let slice: &[i32] = &a3[1..3];
    println!("a3's slice is {:?}", slice);

    example();
}
