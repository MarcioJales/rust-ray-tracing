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
    
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f64 / aspect_ratio) as i64).max(1);

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

    // Camera definitions 
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);

    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let focal_length_vector = Vec3(0.0, 0.0, focal_length);

    let viewport_upper_left = camera_center - focal_length_vector - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;


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

/* More about "impl Trait": https://doc.rust-lang.org/reference/types/impl-trait.html */
fn ray_color<T: Hittable>(r: Ray, world: &T) -> Vec3 {
    let mut hit_record: HitRecord = Default::default();
    if world.hit(r, Interval(0.0, INFINITY), &mut hit_record) {
        return 0.5 * (hit_record.normal + Vec3(1.0, 1.0, 1.0))
    }

    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);

    // Calculate the "lerp". a = 1, color = blue. a = 0, color = blue
    let white = Vec3(1.0, 1.0, 1.0);

    let blue = Vec3(0.5, 0.7, 1.0);

    let blended_value = (1.0 - a) * white + a * blue;
    blended_value
} 