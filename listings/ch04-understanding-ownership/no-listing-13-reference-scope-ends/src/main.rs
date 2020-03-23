fn main() {
    // ANCHOR: here
    let mut s = String::from("witaj");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 i r2 już więcej nie będą używane

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // ANCHOR_END: here
}
