pub struct Breakfast {
    //struct can be public, but not necessarily their fields!! (good)
    pub toast: String,
    seasonal_fruit : String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
        seasonal_fruit: String::from("peaches")
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad
}

fn fix_incorrect_order(){
    cook_order();
    super::serve_order(); //parent module
}

fn cook_order() {}