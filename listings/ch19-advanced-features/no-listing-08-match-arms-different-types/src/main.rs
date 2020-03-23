fn main() {
    let guess = "3";
    // ANCHOR: here
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "witaj",
    };
    // ANCHOR_END: here
}
