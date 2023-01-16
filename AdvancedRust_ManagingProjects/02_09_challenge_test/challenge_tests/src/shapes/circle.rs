use crate::shapes::feature::Feature;

pub struct Circle {
    radius: f64
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {radius}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_circumference()
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn check_circle_funcs(){
        let circle = Circle::new(3.0);
        let area = circle.get_feature(Feature::Area);
        let perimeter = circle.get_feature(Feature::Perimeter);

        assert_eq!(28.274333882308138, area);
        assert_eq!(18.84955592153876, perimeter);
    }
}