mod front_of_house;
pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // If we make a struct public, we still need to set each of its
    // attributes to public or not
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

    // Here both are public even without setting them
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_resturant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our order
    meal.toast = String::from("Wheat");
    println!("toast order: {}", meal.toast);

    // The next line won't compile since we are not allowed
    // to see or modify the seasonal fruit
    // meal.seasonal_fruit = String::from("blueberries");

    let appetizer = back_of_house::Appetizer::Soup;
}
