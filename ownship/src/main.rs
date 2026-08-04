mod borrowing;

use borrowing::*;

// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
fn main() {
    borrow();
    immutable_borrow();
    let mut hello_string = String::from("hello");
    change(&mut hello_string);
    println!("{}", hello_string);
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5; // x 进入作用域

    makes_copy(x); // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn _plus_one() -> i32 {
    // 这段代码并没有发生所有权的转移，原因很简单： 代码首先将 5 绑定到变量 x，接着拷贝 x 的值赋给 y，最终 x 和 y 都等于 5
    // 因为整数是 Rust 基本数据类型，是固定大小的简单值，因此这两个值都是通过自动拷贝的方式来赋值的，都被存在栈中，完全无需在堆上分配内存。
    // 整个过程中的赋值都是通过值拷贝的方式完成（发生在栈中），因此并不需要所有权转移。
    let x = 5;
    let y = x;
    y + 1
}
