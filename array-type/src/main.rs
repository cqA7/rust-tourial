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
    // 所有权发生转移，在下一节的 for 循环汇总会有解释
    // 对于实现了 `copy` 特征的数组（例如 [i32; 10]）而言， `for item in arr` 并不会把 `arr` 的所有权转移，而是直接对其进行了拷贝，因此循环之后仍然可以使用 `arr` 。
    // for string in arr_string {
    //     println!("{}", string);
    // }
    for string in arr_string.iter() {
        println!("{}", string);
    }
    println!("arr_string is {:#?}", arr_string);

    for i in a {
        println!("array a item is {}", i);
    }
    println!("array a is {:?}", a);

    let a3: [i32; 5] = [9, 8, 7, 6, 5];
    let slice: &[i32] = &a3[1..3];
    println!("a3's slice is {:?}", slice);

    example();
}
