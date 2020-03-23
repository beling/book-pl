fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // dangle zwraca referencję do Stringa

    let s = String::from("witaj"); // s to nowy String

    &s // zwracamy referencję do Stringa s
} // Tutaj s wychodzi z zasięgu i jest upuszczone. Pamięć znika.
  // Niebezpieczeństwo!
// ANCHOR_END: here
