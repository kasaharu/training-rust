struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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

    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1
    };
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, origin: {}", black.0, origin.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
