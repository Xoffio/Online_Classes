fn main() {
    crate::hello::english();            // Hello - Absolute
    //crate::hello::spanish();            // Hola  - Absolute
    //hello::spanish();                   // Hola  - Relative
    //crate::hello::casual::english();    // Hi    - Absolute
}

// Defining inline module hello
mod hello{
    pub fn english(){
        println!("Hello");
        spanish();                      // Hola  - Relative
        casual::english();              // Hi    - Relative
    }

    fn spanish(){
        println!("Hola");
    }

    // Defining another module inside the module hello
    // You need to make this module public if you want access it from a higher level
     mod casual{
        pub fn english(){
            println!("Hey");
            crate::hello::spanish();    // Hola  - Absolute
            super::spanish();           // Hola  - Relative
        }
    }
}