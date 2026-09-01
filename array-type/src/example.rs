/// example 例子
/// # 数组的例子
pub fn example() {
    // 编译器自动推导出 one 的类型
    let one = [1, 2, 3];
    // 显式标注类型
    let two: [u8; 3] = [4, 5, 6];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let array: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &array {
        println!("<========================>");
        println!("{:?}", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            println!("{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        // 0..a.len, 是一个 Rust 的语法糖，其实就等于一个数组，元素是从 0,1,2 一直增加到到 a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("{:?} sum is {}", a, sum);
        println!("<========================>");
    }

    println!("{:#?}", array);
}
