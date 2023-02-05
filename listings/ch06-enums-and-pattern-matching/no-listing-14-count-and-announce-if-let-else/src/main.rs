#[derive(Debug)]
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

fn main() {
    let coin = Coin::Penny;
    // ANCHOR: here
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("Ćwiartka ze stanu {:?}!", state);
    } else {
        count += 1;
    }
    // ANCHOR_END: here
}
