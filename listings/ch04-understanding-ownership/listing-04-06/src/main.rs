fn main() {
	let s = String::from("witaj");

	change(&s);
}

fn change(some_string: &String) {
	some_string.push_str(", świecie");
}
