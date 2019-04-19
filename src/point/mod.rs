use crate::metrizable::Metrizable;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl Metrizable for Point {
    fn cost(&self, other: &Point) -> f64 {
        return ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt();
    }
}
