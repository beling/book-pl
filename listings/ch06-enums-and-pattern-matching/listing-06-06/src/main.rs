fn main() {
    // ANCHOR: here
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Maksimum jest ustawione na {}", max),
        _ => (),
    }
    // ANCHOR_END: here
}
