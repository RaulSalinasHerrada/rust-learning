//this is crate::
fn serve_order() {}
//nested paths, for avoid repetition
use rand::{Rng, CryptoRng, Error}; //clean!

use std::io::{self, Write}; //brings std::io and std::io::write!

use rand::*; //bring all

mod front_of_house; //get contents from file and folders with name
mod back_of_house; //get contents form file

pub use front_of_house::hosting::add_to_waitlist;
// external code can make use of the use here!!

use self::front_of_house::hosting; //brings the path so it's more readable

pub fn eat_at_restaurant() {

    // rule: everything is private from the perspective of the parent module
    // child modules can see everything on parent modules

    //needs to be pub as it's seen from a parent perspective (crate)
    //to a child(front of house::serving)

    crate::front_of_house::hosting::add_to_waitlist(); //absolute path, starts at root
    front_of_house::hosting::add_to_waitlist(); //relative, start from current module

    hosting::add_to_waitlist(); //just from the use keyword!
    //good practise is to bring the parent module
    //if enums or structs are brought, use full path

    let mut meal = back_of_house::Breakfast::summer("Rye");
    //struct need to be pub, as well as the funcion (we are on parent mod)

    meal.toast = String::from("Wheat");

    //elements of enum are pub if enum is pub

    let soup = back_of_house::Appetizer::Soup;
    let salad = back_of_house::Appetizer::Salad; 

    let secret_n_orders = rand::thread_rng().gen_range(0..10);

}


// For later :) 

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
