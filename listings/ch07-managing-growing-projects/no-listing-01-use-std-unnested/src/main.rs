use rand::Rng;
// ANCHOR: here
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// ANCHOR_END: here

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Sekretna liczba to: {}", secret_number);

    println!("Podaj swoją liczbę:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Błąd wczytania linii");

    println!("Wybrana przez ciebie liczba: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Za mała!"),
        Ordering::Greater => println!("Za duża!"),
        Ordering::Equal => println!("Jesteś zwycięzcą!"),
    }
}
