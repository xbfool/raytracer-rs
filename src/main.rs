extern crate nalgebra as na;

use na::{Vector3};

type Color = Vector3<f64>;
type Point3 = Vector3<f64>;

fn write_color(color :&Color){
    print!("{} {} {}\n", (255.999 * color.x) as i32,
           (255.999 * color.y) as i32,
           (255.999 * color.z) as i32)
}
fn main() {
    //println!("Hello, world!");

    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {}", image_height - j - 1);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = (image_height - j - 1) as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let color = Color::new(r,g, b);

            write_color(&color);
        }
        print!("\n")
    }

    eprintln!("done")
}
