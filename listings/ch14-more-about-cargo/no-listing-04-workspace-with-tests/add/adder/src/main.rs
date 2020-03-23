use add_one;

fn main() {
    let num = 10;
    println!(
        "Witaj, świecie! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
