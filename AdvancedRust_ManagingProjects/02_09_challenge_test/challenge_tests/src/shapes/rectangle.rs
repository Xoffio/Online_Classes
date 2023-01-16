use crate::shapes::feature::Feature;
pub struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {width, height}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn check_rectangle_funcs(){
        let rect = Rectangle::new(1.0, 2.0);
        let area = rect.get_feature(Feature::Area);
        let perimeter = rect.get_feature(Feature::Perimeter);

        assert_eq!(2.0, area);
        assert_eq!(6.0, perimeter);
    }
}