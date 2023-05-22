fn main() {
    let username = String::from("Satoshi");
    let email = String::from("satoshi@gmx.de");
    let user1 = build_user(username, email);
    let user2 = User {
        email: String::from("another@example.org"),
        ..user1
    };
    println!("{} {} {}", user2.username, user2.email, user2.active);
}

fn build_user (username: String, email:String) -> User {
    User {
        active: true,
        username: username,
        email: email,
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
}