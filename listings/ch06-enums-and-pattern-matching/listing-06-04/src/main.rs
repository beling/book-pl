// ANCHOR: here
#[derive(Debug)] // byśmy mogli za chwilę zobaczyć stan
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// ANCHOR_END: here

fn main() {}
