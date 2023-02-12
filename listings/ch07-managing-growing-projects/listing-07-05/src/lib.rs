mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Ścieżka bezwzględna
    crate::front_of_house::hosting::add_to_waitlist();

    // Ścieżka względna
    front_of_house::hosting::add_to_waitlist();
}
