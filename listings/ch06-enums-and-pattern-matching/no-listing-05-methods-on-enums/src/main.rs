fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // ANCHOR: here
    impl Message {
        fn call(&self) {
            // ciało metody byłoby tutaj zdefiniowane
        }
    }

    let m = Message::Write(String::from("witaj"));
    m.call();
    // ANCHOR_END: here
}
