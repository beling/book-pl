fn main() {
	let s1 = String::from("witaj");

	let len = calculate_length(&s1);

	println!("Długość '{}' wynosi {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { // s jest referencją do Stringa
	s.len()
} // Tu kończy się zakres życia s. Ale ponieważ s nie posiada na własność tego
  // na co wskazuje, nic się nie dzieje.
// ANCHOR_END: here
