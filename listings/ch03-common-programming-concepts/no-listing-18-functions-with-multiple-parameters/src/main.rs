fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Wynik pomiaru: {value}{unit_label}");
}
