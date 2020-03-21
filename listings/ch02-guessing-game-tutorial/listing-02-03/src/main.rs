// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Zgadnij liczbę!");

    // ANCHOR: ch07-04
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // ANCHOR_END: ch07-04

    println!("Sekretna liczba to: {}", secret_number);

    println!("Podaj swoją liczbę:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Błąd wczytania linii");

    println!("Wybrana przez ciebie liczba: {}", guess);
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
