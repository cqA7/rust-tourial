mod user;
mod user_service;

use user::User;
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
}
