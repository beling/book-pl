fn main() {
    let string = no_dangle();
}

// ANCHOR: here
fn no_dangle() -> String {
    let s = String::from("witaj");

    s
}
// ANCHOR_END: here
