struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn example_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 email is {}, username is {}", user1.email, user1.username);

    let user2 = build_user(String::from("someone@example.com"), String::from("username123"));

    println!("user2 email is {}, username is {}", user2.email, user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}