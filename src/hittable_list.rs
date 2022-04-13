use crate::prelude::*;

pub struct Hittable_list {
    pub objects : Vec<Box<dyn Hittable>>
}

impl Hittable_list {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>){
        self.objects.push(object);
    }
}

impl Hittable for Hittable_list {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        todo!()
    }
}