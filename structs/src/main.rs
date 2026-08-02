use std::fmt::{self, Display};

fn main() {
    let user1 = build_user(String::from("tommywagner"), String::from("tommywagner@email.com"));

    let user2 = User {
        username: String::from("TomWagner"),
        email: String::from("tw123@email.com"),
        ..user1
    };

    println!("user1 name: {}", user1.username);
    println!("user2 name: {}", user2.username);
    // println!("user1: {}", user1);

    let grey = Color(100, 100, 100);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: true,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u32, u32, u32);
