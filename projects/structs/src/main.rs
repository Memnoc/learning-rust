struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 5,
    }
}

fn main() {
    println!("****** Struct ******");

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);
    println!("Is active: {}", user1.active);
    println!("Email: {}", user1.email);
    println!("Is signed in: {}", user1.sign_in_count);

    let mut user1 = User {
        active: false,
        username: String::from("someusername345"),
        email: String::from("someoneagain@example.com"),
        sign_in_count: 3,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("Mutable user one email: {}", user1.email);

    // using the function
    let email = String::from("function@example.com");
    let username = String::from("LuckyDuck");
    let user = build_user(email, username);
    println!(
        "User: {}, Email: {}, Active: {}, Sign-in: {}",
        user.username, user.email, user.active, user.sign_in_count
    );
}
