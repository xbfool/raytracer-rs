use nalgebra::Vector3;

pub type Point = nalgebra::Point<f64, 3>;
pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>
}

impl Ray {
    fn at(&self) -> Point {
        return self.origin + self.dir;
    }
}