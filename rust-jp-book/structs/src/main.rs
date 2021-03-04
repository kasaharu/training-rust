struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
