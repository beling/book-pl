fn main() {
    // ANCHOR: here
    {
        let s = String::from("witaj"); // s jest ważna od tego momentu

        // jakieś operacje na s
    }                                  // bieżący zasięg się kończy - s traci
                                       // ważność
    // ANCHOR_END: here
}
