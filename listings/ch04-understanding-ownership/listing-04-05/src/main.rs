fn main() {
    let s1 = String::from("witaj");

    let (s2, len) = oblicz_dlugosc(s1);

    println!("Długość '{}' wynosi {}.", s2, len);
}

fn oblicz_dlugosc(s: String) -> (String, usize) {
    let dlugosc = s.len(); // len() zwraca długość łańcucha znaków.

    (s, dlugosc)
}

