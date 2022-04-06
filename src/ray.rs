use nalgebra::Vector3;

pub type Point = nalgebra::Vector3<f64>;
pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        return self.origin + self.dir * t;
    }
}