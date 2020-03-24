fn main() {
    // ANCHOR: here
    let mut s = String::from("witaj");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // DUŻY PROBLEM

    println!("{}, {}, i {}", r1, r2, r3);
    // ANCHOR_END: here
}
