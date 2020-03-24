use std::io;

fn main() {
    println!("Zgadnij numer!");

    println!("Podaj swoją liczbę:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess);

    println!("Wybrana przez ciebie liczba: {}", guess);
}
