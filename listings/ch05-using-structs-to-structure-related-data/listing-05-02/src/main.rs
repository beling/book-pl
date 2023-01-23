struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn main() {
    let user1 = User {
        active: true,
        username: String::from("jakisusername"),
        email: String::from("ktos@example.com"),
        sign_in_count: 1,
    };
}
// ANCHOR_END: here
