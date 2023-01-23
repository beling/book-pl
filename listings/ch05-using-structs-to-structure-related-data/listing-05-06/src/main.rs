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
        active: user1.active,
        username: user1.username,
        email: String::from("kolejny@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// ANCHOR_END: here
