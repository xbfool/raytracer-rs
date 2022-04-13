mod ray;
mod sphere;
mod hittable_list;


pub mod prelude {
    pub use std::ops::Div;
    extern crate nalgebra as na;

    pub use na::{Vector3, Point3};
    pub use crate::ray::*;

    pub type Color = Vector3<f64>;

    pub fn write_color(color :&Color){
        print!("{} {} {}\n", (255.999 * color.x) as i32,
               (255.999 * color.y) as i32,
               (255.999 * color.z) as i32)
    }



    pub fn hit_sphere(center:Vector3<f64>, radius: f64, r: &Ray) -> f64{
        let oc = &r.origin - center;
        let a = r.dir.dot(&r.dir);
        let half_b = oc.dot(&r.dir);
        let c = oc.dot(&oc) -
            &radius * &radius;
        let discriminant = half_b * half_b  - a * c;
        if discriminant < 0. {
            return -1.0
        }else{
            return (-half_b - discriminant.sqrt())/ a
        }
    }

    pub fn ray_color(ray: &Ray) -> Color{
        let t =  hit_sphere(Vector3::new(0., 0., -1.), 0.5, ray);
        if t > 0.0 {
            let n = (ray.at(t) - Vector3::new(0., 0., -1.)).normalize();
            return 0.5 * Color::new(n[0] + 1., n[1] + 1., n[2] + 1.);
        }
        let unit_direction = ray.dir.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) +
            t * Color::new(0.5, 0.7, 1.0)
    }

}

use crate::prelude::*;

fn main() {
    //println!("Hello, world!");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32 ;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Vector3<f64> = Vector3::new(0., 0., 0.);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal.div(2.0) - vertical.div(2.0) - Vector3::new(0., 0., focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j - 1);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray{
                origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin
            };
            let color = ray_color(&r);

            write_color(&color);
        }
        print!("\n")
    }

    eprintln!("done")
}
