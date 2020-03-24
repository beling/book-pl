struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "ktos@example.com",
        username: "jakisusername123",
        active: true,
        sign_in_count: 1,
    };
}
