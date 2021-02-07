// ANCHOR: here
struct QuitMessage; // struktura-jednostka
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // struktura-krotka
struct ChangeColorMessage(i32, i32, i32); // struktura-krotka
                                          // ANCHOR_END: here

fn main() {}
