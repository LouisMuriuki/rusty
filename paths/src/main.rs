// no need to add the pub keyword here as the modules is siblings with the function that will call it.
mod front_of_house {
    // items inside module are private to that module only unless pub
    pub mod hosting {
        // without pub keyword we cant access this from eat_at_restaurant fn
        pub fn add_to_waitlist() { // as well as make this item pub inorder for it to be accessed
        }
    }
}


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

    fn fix_incorrect_order() {
        cook_order();
        //super allows us to refernce an item in the parent module
        super::deliver_order(); //parent module of back_of_house has deliver_order fn
    }

    fn cook_order() {}
}
// Absolute path-begins with literal crate-full path
//     crate::front_of_house::hosting::add_to_waitlist();
//relative path
//     front_of_house::hosting::add_to_waitlist()

//use keyword
use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye"); //relative path
                                                            // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

fn deliver_order() {}

fn main() {
    println!("Hello, world!");
    eat_at_restaurant()
}

