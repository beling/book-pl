use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // ANCHOR: here
    loop {
        // --snip--

        // ANCHOR_END: here
        println!("Podaj swoją liczbę:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Błąd wczytania linii");

        // ANCHOR: here
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
            // ANCHOR_END: here
            Ordering::Less => println!("Za mała!"),
            Ordering::Greater => println!("Za duża!"),
            Ordering::Equal => {
                println!("Jesteś zwycięzcą!");
                break;
            }
        }
        // ANCHOR: here
    }
    // ANCHOR_END: here
}
