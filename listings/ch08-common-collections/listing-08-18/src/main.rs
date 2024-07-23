fn main() {
    // ANCHOR: here
    let s1 = String::from("Witaj, ");
    let s2 = String::from("Świecie!");
    let s3 = s1 + &s2; // zauważ, że s1 został tutaj przeniesiony i nie możemy go użyć ponownie
                       // ANCHOR_END: here
}
