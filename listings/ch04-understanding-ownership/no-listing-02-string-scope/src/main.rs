fn main() {
    // ANCHOR: here
    {
        let s = String::from("witaj"); // s ma ważność od tego momentu

        // jakieś operacje na s
    }                                  // bieżący zasięg się kończy - s traci
                                       // ważność
    // ANCHOR_END: here
}
