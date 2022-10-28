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

// Lets the compiler know that it needs to use the file front_of_house.rs
mod front_of_house;

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

// Allows for the use of shortened path specification, as seen under comment 1
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // Shortest way to specify path (1)
    // Can only be used in functions of the same scope, such as eat_at_restaurant
    // This statement would not be valid in the back_of_house module
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// Practical use case of idiomatic specification
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/

/*
How to specify two types of the same name:

use std::fmt::Result;
use std::io::Result as IoResult;
*/

/*
Adding a pub keyword before the use keyword allows external code to use that path as well,
if pub use crate::front_of_house::hosting was declared instead of statement 1 then in an external 
file you could declare restaurant::hosting::add_to_waitlist()
*/

/*
External code that is not a part of the standard library must be defined in the cargo.toml file
*/

/*
Brings all items from std::collections into scope
use std::collections*;
*/

/*
Single line declaration bringing two seperate modules into scope
use std::io::{self, Write};
*/