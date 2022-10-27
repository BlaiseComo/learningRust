/* pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
} */

// Defining a module
// Alternative way to view this module
/*
crate
    front_of_house
        hosting
            add_to_waitlist
            seat_at_table
        serving
            take_order
            serve_order
            take_payment
*/

mod front_of_house {
    // First submodule
    pub mod hosting {

        // Parent module and function must both be public to be accessed by other functions such as eat_at_restaurant
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Second submodule
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {

    // Public struct with one public attribute
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

    // Public enum declarations mean that all values of the enum are public unlike structs
    // Struct attributes are declared as public on a case by case basis

    fn fix_incorrect_order() {
        cook_order();

        // super searches the parent module of back_of_house which is crate
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}