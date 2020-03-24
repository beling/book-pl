fn main() {
    // ANCHOR: here
    {
                                // s nie jest tu ważna - jeszcze jej nie zadeklarowano
        let s = "witaj";   // od tego momentu s jest ważna

        // jakieś operacje na s
    }                           // bieżący zasięg się kończy - s traci ważność
    // ANCHOR_END: here
}