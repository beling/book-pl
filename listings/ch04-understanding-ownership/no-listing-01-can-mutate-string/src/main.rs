fn main() {
    // ANCHOR: here
    let mut s = String::from("witaj");

    s.push_str(", świecie!"); // push_str() dodaje literał do zmiennej String

    println!("{}", s); // To spowoduje wyświetlenie tekstu: `hello, world!`
                       // ANCHOR_END: here
}
