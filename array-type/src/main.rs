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
}
