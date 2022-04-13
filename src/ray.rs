use nalgebra::Vector3;


pub type Point = nalgebra::Vector3<f64>;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>){
        self.front_face = r.dir.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        }else{
            -outward_normal.clone()
        }
    }
}

pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        return self.origin + self.dir * t;
    }
}

pub trait Hittable {
    fn hit(&self, r : &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}