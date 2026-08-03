fn main() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@example.com");

    let user2 = User {
        email: String::from("tt@ww.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:#?}", user1);
    println!("{:#?}", user2);
}
