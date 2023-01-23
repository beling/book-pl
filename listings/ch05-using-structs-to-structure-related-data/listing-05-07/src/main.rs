struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let user1 = User {
        email: String::from("ktos@example.com"),
        username: String::from("jakisusername123"),
        active: true,
        sign_in_count: 1,
    };
    // ANCHOR: here

    let user2 = User {
        email: String::from("kolejny@example.com"),
        ..user1
    };
}
// ANCHOR_END: here
