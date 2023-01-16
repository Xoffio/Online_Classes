pub mod shapes;

#[cfg(test)]
mod tests {
    use super::shapes::rectangle::*;
    use super::shapes::circle::*;
    use super::shapes::feature::*;

    #[test]
    fn test_area_mul () {
        let rect = Rectangle::new(1.0, 2.0);
        let circ = Circle::new(3.0);

        let mul_result = 28.274333882308138 * 2.0;
        assert_eq!(mul_result, circ.get_feature(Feature::Area) * rect.get_feature(Feature::Area));
    }

    #[test]
    fn rand_test_area_mul () {
        let w = rand::random::<f64>();
        let h = rand::random::<f64>();
        let radius = rand::random::<f64>();

        let rect = Rectangle::new(w, h);
        let circ = Circle::new(radius);

        let rand_circ_area = std::f64::consts::PI * radius.powi(2);
        let rand_rect_area = w*h;

        let mul_result = rand_circ_area * rand_rect_area;
        assert_eq!(mul_result, circ.get_feature(Feature::Area) * rect.get_feature(Feature::Area));
    }
}