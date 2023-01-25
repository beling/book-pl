fn main() {
    // ANCHOR: here
    {
                           // s nie ma tu jeszcze ważności - jeszcze jej nie zadeklarowano
        let s = "witaj";   // od tego momentu s ma ważność

        // jakieś operacje na s
    }                           // bieżący zasięg się kończy - s traci ważność
    // ANCHOR_END: here
}