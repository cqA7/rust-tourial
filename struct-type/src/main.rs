mod user;
mod user_service;

use user::{Point, User};
use user_service::build_user;

fn main() {
    let mut user = User {
        name: String::from("Tom"),
        age: 10,
    };
    user.name = String::from("jerry");
    println!("user name is {}, user age is {}", user.name, user.age);

    let name = String::from("tesla");
    let tesla_user = build_user(name, 10);
    println!("Tesla's name is {}", tesla_user.name);

    let user1 = build_user(String::from("user1"), 10);
    let user2 = User { age: 20, ..user1 };
    println!("user2 is {:#?}", user2);
    // 下面两行都会报错，因为 name 字段的所有权发生转移
    // println!("user1 is {:#?}", user1);
    // println!("user1 is {:#?}", user1.name);
    // name 所有权被转移给了 user2，导致了 user1 无法再被使用，但是并不代表 user1 内部的其它字段不能被继续使用
    println!("user1's age is {:#?}", user1.age);

    let point = Point(1, 2, 3);
    println!("x, y, z is {}, {}, {}", point.0, point.1, point.2);
}
