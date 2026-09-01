use crate::struct_enum;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // abs path
    struct_enum::module_demo::front_of_house::hosting::add_to_waitlist();
}

/* The super keyword */
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

/* The back of house */
mod private_kitchen {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            return Breakfast { toast: String::from(toast), seasonal_fruit: String::from("apple") };
        }
    }

    /* Enums are only useful if they are made public */
    pub enum Appetizer {Soup, Salad}
}

pub fn eat_breakfast() {
    let mut my_apple_breakfast = private_kitchen::Breakfast::summer("Rye toast");
    my_apple_breakfast.toast = String::from("Wheat toast");

    // Error on this line because private
    // my_apple_breakfast.seasonal_fruit = String::from("blueberries"); 
}

