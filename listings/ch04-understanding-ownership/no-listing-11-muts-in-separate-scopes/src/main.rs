fn main() {
    // ANCHOR: here
    let mut s = String::from("witaj");

    {
        let r1 = &mut s;
    } // tu kończy się czas życia r1, więc odtąd możemy bez problemu utworzyć kolejną referencję

    let r2 = &mut s;
    // ANCHOR_END: here
}
