pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adding to waitlist...!");
        }

        pub fn seat_at_table() {
            println!("Seating at table");
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("Taking order");
        }

        pub fn server_order() {
            println!("Serving order...");
        }

        pub fn take_payment() {
            println!("Serving order...");
        }
    }
}

use crate::front_of_house::hosting;

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    //Order a breakfast in the summer with Rye toast
    hosting::add_to_waitlist();
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    //
    // // Change out mind about what bread we'd like
    // meal.toast = String::from("Wheat");
    // println!("I'd like {} toast please", meal.toast);

    //The next line won't compile if we uncomment it; we're not allowed
    //to see or modify the seasonal fruit that comes with the meal
    //meal._seasonal_fruit -= String::from("blueberries");
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
