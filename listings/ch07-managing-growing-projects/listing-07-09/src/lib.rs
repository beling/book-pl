mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Zamów śniadanie w lecie z tostem żytnim
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Zmiana zdania na temat tego, jaki chleb chcemy
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Następna linia nie skompiluje się, jeśli ją odkomentujemy;
    // nie możemy bowiem przeglądać ani modyfikować sezonowych owoców,
    // które są dołączone do posiłku
    // meal.seasonal_fruit = String::from("blueberries");
}
