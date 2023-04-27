use std::fs::File;
use std::io::ErrorKind;

fn main() {

    //--------------package with binary crate--------------
    // creation
    // $ cargo new my-project
    // file structure
    // backyard
    //     ├── Cargo.lock
    //     ├── Cargo.toml
    //     └── src
    //         ├── garden
    //         │   └── vegetables.rs
    //         ├── garden.rs
    //         └── main.rs
    // file: src/garden.rs : 
    //      pub mod vegetables;
    // file: src/garden/vegetables.rs :
    //      #[derive(Debug)]
    //      pub struct Asparagus {}
    // file: src/main.rs
    //      use crate::garden::vegetables::Asparagus;
    //      pub mod garden;
    //      fn main() {
    //          let plant = Asparagus {};
    //          println!("I'm growing {:?}!", plant);
    //      }
    
    //--------------package with library crate--------------
    // creation
    // cargo new restaurant --lib
    // file structure
    // crate
    // └── front_of_house
    //         ├── hosting
    //         │   ├── add_to_waitlist
    //         │   └── seat_at_table
    //         └── serving
    //               ├── take_order
    //               ├── serve_order
    //               └── take_payment
    // file: src/lib.rs :
    //     mod front_of_house {
    //         mod hosting {
    //             fn add_to_waitlist() {}
    //             fn seat_at_table() {}
    //         }
    //         mod serving {
    //             fn take_order() {}
    //             fn serve_order() {}
    //             fn take_payment() {}
    //         }
    //     }
    
    //--------------absolute path--------------
    // crate::front_of_house::hosting::add_to_waitlist();
    
    //--------------relative path--------------
    // front_of_house::hosting::add_to_waitlist();
    
    //--------------pub keyword--------------
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();
        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
    
    //--------------super keyword--------------
    fn deliver_order() {}
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::deliver_order();
        }
        fn cook_order() {}
    }
    
    //--------------public struct--------------
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
    }
    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
    
    //--------------public enum--------------
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }
    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
    
    //--------------use keyword--------------
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    // A use statement only applies in the scope it’s in
    // mod front_of_house {
    //     pub mod hosting {
    //         pub fn add_to_waitlist() {}
    //     }
    // }
    // use crate::front_of_house::hosting;
    // mod customer {
    //     pub fn eat_at_restaurant() {
    //         hosting::add_to_waitlist();
    //     }
    // }
    
    //--------------idiomatic use paths--------------
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    use crate::front_of_house::hosting::add_to_waitlist;
    pub fn eat_at_restaurant() {
        add_to_waitlist();
    }
    
    //--------------changing names with as keyword--------------
    use std::fmt::Result;
    use std::io::Result as IoResult;
    fn function1() -> Result {
        // --snip--
    }
    fn function2() -> IoResult<()> {
        // --snip--
    }
    
    //--------------re-exporting names with pub use keyword--------------
    mod front_of_house {
        pub mod hosting {
           pub fn add_to_waitlist() {}
        }
    }
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    
    //--------------using external packages--------------
    // filename: Cargo.toml:
    //      rand = "0.8.5"
    use rand::Rng;
    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }
    
    //--------------nested paths--------------
    use std::{cmp::Ordering, io};
    
    //--------------glob operator--------------
    use std::collections::*;
    
    //--------------separating modules into different files--------------
    // ex 1
    // Filename: src/lib.rs:
    mod front_of_house;
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    // Filename: src/front_of_house.rs:
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    // ex 2
    // Filename: src/lib.rs:
    mod front_of_house;
    pub use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
    // Filename: src/front_of_house.rs:
    pub mod hosting;
    // Filename: src/front_of_house/hosting.rs:
    pub fn add_to_waitlist() {}

}


