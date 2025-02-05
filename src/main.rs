use std::f64::INFINITY;
use std::{fs::File, rc::Rc};

mod vec3;
mod ray;
mod hittable; 
mod sphere;
mod interval;
mod camera;
mod random;
mod material;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use interval::Interval;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;
use ray::Ray;

#[macro_use]
extern crate assert_float_eq;

fn main() {
    // World
    let mut world: HittableList = Default::default();

    let material_ground = Rc::new(
        Lambertian {
            albedo: Vec3(0.8, 0.8, 0.0)
        }
    );

    let material_center = Rc::new(
        Lambertian {
            albedo: Vec3(0.1, 0.2, 0.5)
        }
    );

    let material_left = Rc::new(
        Metal {
            albedo: Vec3(0.8, 0.8, 0.8)
        }
    );

    let material_right = Rc::new(
        Metal {
            albedo: Vec3(0.8, 0.6, 0.2)
        }
    );

    world.add(Rc::new(
        Sphere {
            center: Vec3(0.0, 0.0, -1.2),
            radius: 0.5,
            material: material_center
        }
    ));
    world.add(Rc::new(
        Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_ground
        }
    ));
    world.add(Rc::new(
        Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_left
        }
    ));
    world.add(Rc::new(
        Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: material_right
        }
    ));

    let f = File::create("image.ppm").unwrap();

    let mut cam: Camera = Default::default();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(world, f);
}