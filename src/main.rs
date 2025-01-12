use std::f64::INFINITY;
use std::{fs::File, rc::Rc};
use std::io::Write;

mod vec3;
mod ray;
mod hittable; 
mod sphere;
mod interval;
mod camera;

use hittable::{HitRecord, Hittable, HittableList};
use interval::Interval;
use sphere::Sphere;
use vec3::Vec3;
use ray::Ray;

#[macro_use]
extern crate assert_float_eq;

fn main() {
    // World
    let mut world: HittableList = Default::default();
    world.add(Rc::new(
        Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5
        }
    ));
    world.add(Rc::new(
        Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0
        }
    ));

    let mut f = File::create("image.ppm").unwrap();

    writeln!(f, "P3").unwrap();
    writeln!(f, "{image_width} {image_height}").unwrap();
    writeln!(f, "255").unwrap();

    let (mut j, mut i) = (0, 0);

    while j < image_height {
        i = 0;
        println!("\rScanlines remaining: {}", image_height - j);
        while i < image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray{
                orig: camera_center,
                dir: ray_direction
            };

            let pixel_color = ray_color(ray, &world);

            write_color(pixel_color, &mut f);

            i += 1;
        }
        j += 1;
    }

    println!("\rDone.                 ");
}

fn write_color(pixel_color: Vec3, f: &mut File) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i64;
    let gbyte = (255.999 * g) as i64;
    let bbyte = (255.999 * b) as i64;

    writeln!(f, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}