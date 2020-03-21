use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Sekretna liczba to: {}", secret_number);

    loop {
        println!("Podaj swoją liczbę:");

        let mut guess = String::new();

        // ANCHOR: here
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Błąd wczytania linii");

        // ANCHOR: ch19
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // ANCHOR_END: ch19

        println!("Wybrana przez ciebie liczba: {}", guess);

        // --snip--
        // ANCHOR_END: here

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mała!"),
            Ordering::Greater => println!("Za duża!"),
            Ordering::Equal => {
                println!("Jesteś zwycięzcą!");
                break;
            }
        }
    }
}
