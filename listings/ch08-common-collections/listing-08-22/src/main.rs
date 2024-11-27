fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Ulubiony kolor");
    let field_value = String::from("Niebieski");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name i field_value są w tym momencie unieważnione, spróbuj ich użyć
    // i zobacz, jaki błąd kompilatora otrzymasz!
    // ANCHOR_END: here
}
