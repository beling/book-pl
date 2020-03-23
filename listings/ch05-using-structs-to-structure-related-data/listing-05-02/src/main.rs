struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // ANCHOR: here
    let user1 = User {
        email: String::from("ktos@example.com"),
        username: String::from("jakisusername"),
        active: true,
        sign_in_count: 1,
    };
    // ANCHOR_END: here
}
