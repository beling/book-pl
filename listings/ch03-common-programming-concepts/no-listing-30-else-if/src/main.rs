fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number jest podzielny przez 4");
    } else if number % 3 == 0 {
        println!("number jest podzielny przez 3");
    } else if number % 2 == 0 {
        println!("number jest podzielny przez 2");
    } else {
        println!("number nie jest podzielny przez 4, 3, ani 2");
    }
}
