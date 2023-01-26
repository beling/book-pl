// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("witaj świecie");

    // `first_word` pracuje na wycinkach obejmujących `String`i w części lub całości
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` pracuje też na referencjach do `String`ów, które są równoważne
    // wycinkom obejmującym całość tych `String`ów
    let word = first_word(&my_string);

    let my_string_literal = "witaj świecie";

    // `first_word` pracuje też na wycinkach literałów łańcuchowych
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Ponieważ literały łańcuchowe *są* równocześnie wycinkami łańcuchów,
    // to też zadziała, i to bez składni tworzącej wycinek!
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
