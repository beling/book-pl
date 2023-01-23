fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Wartość x w wewnętrznym bloku kodu wynosi: {x}");
    }

    println!("Wartość x wynosi: {x}");
}
