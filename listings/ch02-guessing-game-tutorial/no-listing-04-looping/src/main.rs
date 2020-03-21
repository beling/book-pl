use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // ANCHOR: here
    // --snip--

    println!("Sekretna liczba to: {}", secret_number);

    loop {
        println!("Podaj swoją liczbę:");

        // --snip--

        // ANCHOR_END: here

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Błąd wczytania linii");

        let guess: u32 = guess.trim().parse().expect("Podaj liczbę!");

        println!("Wybrana przez ciebie liczba: {}", guess);

        // ANCHOR: here
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mała!"),
            Ordering::Greater => println!("Za duża!"),
            Ordering::Equal => println!("Jesteś zwycięzcą!"),
        }
    }
}
// ANCHOR_END: here
