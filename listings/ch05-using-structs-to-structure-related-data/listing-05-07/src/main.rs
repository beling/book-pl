struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("ktos@example.com"),
        username: String::from("jakisusername123"),
        active: true,
        sign_in_count: 1,
    };

    // ANCHOR: here
    let user2 = User {
        email: String::from("kolejny@example.com"),
        username: String::from("kolejnyusername567"),
        ..user1
    };
    // ANCHOR_END: here
}
