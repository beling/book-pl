use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    println!("Podaj swoją liczbę:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Błąd wczytania linii.");

    println!("Wybrana przez ciebie liczba: {}", guess);
}
