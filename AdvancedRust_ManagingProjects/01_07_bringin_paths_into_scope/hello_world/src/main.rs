// "use" brings the 
//use crate::greeting::formal;
//use crate::greeting::casual;
use crate::greeting::{formal, casual};

// You can use `as` to change the name of an item brought into scope
// this way you are able to bring items with the same name without a problem.
use crate::greeting::formal::spanish as f_spanish;
use crate::greeting::casual::spanish as c_spanish;

// Bringing all items in the prelude module of the `rand` crate
// the prelude items are the most used items.
use rand::prelude::*;

// You can also be more specific
use crate::greeting::formal::spanish;

fn main() {
    crate::greeting::formal::english();

    formal::english();
    casual::english();
    spanish();

    println!("--- Using `use` and `as` ---");
    f_spanish();
    c_spanish();

    // using the function `random` from the rand prelude library 
    println!("char: {}", random::<u8>());
}

// Defining inline module greeting
mod greeting{
    pub mod formal{
        pub fn english(){
            println!("Hello");
        }
    
        pub fn spanish(){
            println!("Hola");
        }
    }

    pub mod casual{
        pub fn english(){
            println!("Hey");
        }

        pub fn spanish(){
            println!("Epa");
        }
    }
}