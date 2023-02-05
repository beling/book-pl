// ANCHOR: here
struct QuitMessage; // struktura jednostkowa
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // struktura krotkowa
struct ChangeColorMessage(i32, i32, i32); // struktura krotkowa
                                          // ANCHOR_END: here

fn main() {}
