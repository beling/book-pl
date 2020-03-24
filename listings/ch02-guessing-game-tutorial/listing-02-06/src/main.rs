use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Podaj swoją liczbę:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Błąd wczytania linii");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Wybrana przez ciebie liczba: {}", guess);

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
