fn main() {
    let rect = shape::Rectangle::new(2.0 , 3.5);
    println!("rect area is {}", rect.get_area());
    println!("rect width is {}", rect.width);
}

mod shape {
    // Struct needs to be public to be accessed
    pub struct Rectangle {
        // also each item in it
        pub width: f64,
        height: f64,
    }
    
    // imp does not need to be public
    // but each item in it does.
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height
            }
        }
    
        pub fn get_area(&self) -> f64 {
            self.width * self.height
        }
    }
}