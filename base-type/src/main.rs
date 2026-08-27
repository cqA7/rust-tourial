mod demo;

use demo::{add, char_bool, greet_world, integer, plus_or_minus, statement_expression, variable};

fn main() {
    integer();
    greet_world();
    variable();
    char_bool();
    let y = statement_expression(10);
    println!("y = {}", y);
    let sum = add(10, 4);
    println!("sum is {}", sum);
    let res = plus_or_minus(10);
    println!("plus_or_minus(10) = {}", res);
}
