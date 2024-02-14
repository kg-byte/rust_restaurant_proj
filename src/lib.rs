mod front_of_house;
mod back_of_house;

// pub use: Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// Now that this pub use has re-exported the hosting module from the root module, external code can now use the path restaurant::hosting::add_to_waitlist() instead.
pub use crate::front_of_house::hosting;
pub use crate::back_of_house::{cooking, menu};


pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    hosting::add_to_waitlist();

    // Order a breakfast in summer with Rye toast
    let mut meal = menu::Breakfast::summer("Rye");
    // Change our mind about the bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line won't compile; we're not allowed to see/modify seasonal fruit
    // meal.seasonal_fruit = String::from("Strawberries");

    // Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant.
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}

mod customer {
    // must bring hosting into private mod scope
    // use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
        // or reference shortcut in the parent module
        // parent has access to scope
        super::hosting::add_to_waitlist();
    }
}


// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// //
// use std::cmp::Ordering;
// use std::io;
// // --> simplifies into --->
// use std::{cmp::Ordering, io};
// // 

// //
// use std::io;
// use std::io::Write;
// // --> simplifies into 
// use std::io::{self, Write};
// //


// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
//use std::collections::*;
