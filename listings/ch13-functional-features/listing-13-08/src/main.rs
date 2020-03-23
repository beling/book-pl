fn main() {
    // ANCHOR: here
    let example_closure = |x| x;

    let s = example_closure(String::from("witaj"));
    let n = example_closure(5);
    // ANCHOR_END: here
}
