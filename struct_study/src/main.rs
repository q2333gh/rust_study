struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someone1"),
        email: String::from("somewhere@x.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    let mut user2 = build_user("email001@x.com".to_owned(), "someone002".to_owned());
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}", user3);
    return;
}
