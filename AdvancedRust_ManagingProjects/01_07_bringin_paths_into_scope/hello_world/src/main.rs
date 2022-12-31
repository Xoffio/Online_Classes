// "use" brings the 
//use crate::greeting::formal;
//use crate::greeting::casual;
use crate::greeting::{formal, casual};

// You can also be more specific
use crate::greeting::formal::spanish;

fn main() {
    crate::greeting::formal::english();

    formal::english();
    spanish();
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