fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("witaj świecie");

    let word = first_word(&s); // word otrzyma wartość 5

    s.clear(); // to czyści łańcuch s, czyniąc go równym ""

    // tutaj word wciąż ma wartość 5, ale nie ma już łańcucha
    // dla którego 5 coś znaczy. word zupełnie straciło ważność!
}
// ANCHOR_END: here
