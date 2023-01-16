mod shapes;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::circle::Circle;
use crate::shapes::feature::Feature;
fn main() {
    let rect = Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(Feature::Area);
    println!("rect area is {}", area);

    let circ = Circle::new(3.0);
    let perimeter = circ.get_feature(Feature::Perimeter);
    println!("circ perimeter is {}", perimeter);
}