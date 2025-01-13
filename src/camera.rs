use std::fs::File;
use std::io::Write;

use crate::{Hittable, HitRecord};
use crate::{Interval, INFINITY};
use crate::Vec3;
use crate::Ray;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    image_height: i64,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3
}

impl Camera {
    pub fn render<T: Hittable>(mut self, world: T, mut f: File) {
        self.initialize();

        writeln!(f, "P3").unwrap();
        writeln!(f, "{0} {1}", self.image_width, self.image_height).unwrap();
        writeln!(f, "255").unwrap();
    
        let (mut j, mut i) = (0, 0);
    
        while j < self.image_height {
            i = 0;
            println!("\rScanlines remaining: {}", self.image_height - j);
            while i < self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray{
                    orig: self.camera_center,
                    dir: ray_direction
                };
    
                let pixel_color = Self::ray_color(ray, &world);
    
                Self::write_color(pixel_color, &mut f);
    
                i += 1;
            }
            j += 1;
        }
    
        println!("\rDone.                 ");
    }

    fn initialize(&mut self) {

        // Calculate the image height, and ensure that it's at least 1.
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as i64).max(1);

        // Camera definitions 
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        self.camera_center = Vec3(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);

        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let focal_length_vector = Vec3(0.0, 0.0, focal_length);

        let viewport_upper_left = self.camera_center - focal_length_vector - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) / 2.0;
    
    }

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
}