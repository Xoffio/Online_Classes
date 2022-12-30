fn main() {
    crate::hello::english();            // Hello - Absolute
    crate::hello::spanish();            // Hola  - Absolute
    hello::spanish();                   // Hola  - Relative
    crate::hello::casual::english();    // Hi    - Absolute
}

// Defining inline module hello
mod hello{
    fn english(){
        println!("Hello");
        spanish();                      // Hola  - Relative
        casual::english();              // Hi    - Relative
    }

    fn spanish(){
        println!("Hola");
    }

    // Defining another module inside the module hello
    mod casual{
        fn english(){
            println!("Hey");
            crate::hello::spanish();    // Hola  - Absolute
            super::spanish();           // Hola  - Relative
        }
    }
}