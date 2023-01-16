mod shapes;
use crate::shapes::rectangle::Rectangle;
use crate::shapes::circle::Circle;
use crate::shapes::feature::Feature;
fn main() {
    let rect = Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(Feature::Area);
    let rect_perimeter = rect.get_feature(Feature::Perimeter);
    println!("rect area is {}", area);
    println!("rect perimeter is {}", rect_perimeter);

    let circ = Circle::new(3.0);
    let perimeter = circ.get_feature(Feature::Perimeter);
    let c_area = circ.get_feature(Feature::Area);

    println!("circ perimeter is {}", perimeter);
    println!("circ area is {}", c_area);
}