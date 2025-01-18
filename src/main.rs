use std::f64::INFINITY;
use std::{fs::File, rc::Rc};

mod vec3;
mod ray;
mod hittable; 
mod sphere;
mod interval;
mod camera;
mod random;

use camera::Camera;
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

    let f = File::create("image.ppm").unwrap();

    let mut cam: Camera = Default::default();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(world, f);
}