// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn server_orde() {}

//         fn take_payment() {}
//     }
// }
// https://www.youtube.com/watch?v=5RPXgDQrjio&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&index=7&ab_channel=Let%27sGetRusty

// ----------------------------------------------
// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;
// ----------------------------------------------
// Turned above block into a nested path below

use rand::{ Rng, CryptoRng, ErrorKind::Transient };
use std::io::{ self, Write };

mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist()
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_foh() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_boh() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
}

pub fn eat_at_restaurant_appertizer() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}