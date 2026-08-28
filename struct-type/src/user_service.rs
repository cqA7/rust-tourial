use crate::user::User;
// use crate::User;

pub fn build_user(name: String, age: u8) -> User {
    User { name, age }
}
