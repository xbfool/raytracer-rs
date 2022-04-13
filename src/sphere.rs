use crate::prelude::*;

struct Sphere {
    center: Point,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - self.center;
        let a = r.dir.dot(&r.dir);
        let half_b = oc.dot(&r.dir);
        let c = oc.dot(&oc) -
            self.radius * self.radius;
        let discriminant = half_b * half_b  - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root <t_min || t_max < root {
                return None;
            }
        }

        let ret = HitRecord{
            p: r.at(root),
            normal: (r.at(root) - self.center) / self.radius,
            t: root
        };

        return Some(ret);
    }
}