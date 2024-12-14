use std::fs::File;
use std::io::Write;

mod vec3;
mod ray;

use vec3::Vec3;

#[macro_use]
extern crate assert_float_eq;

fn main() {
    
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f64 / aspect_ratio) as i64).max(1);

    // Camera definitions 
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width/image_height) as f64;
    let camera_center = Vec3 {
        e: [0.0, 0.0, 0.0]
    };


    let mut f = File::create("image.ppm").unwrap();

    writeln!(f, "P3").unwrap();
    writeln!(f, "{image_width} {image_height}").unwrap();
    writeln!(f, "255").unwrap();

    let (mut j, mut i) = (0, 0);

    while j < image_height {
        i = 0;
        println!("\rScanlines remaining: {}", image_height - j);
        while i < image_width {
            let pixel_color = Vec3 {
                e: [(i as f64 / (image_width - 1) as f64 ), (j as f64 / (image_height - 1) as f64), 0.0]
            };

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

fn ray_color() -> Vec3 {
    Vec3 {
        e: [0.0, 0.0, 0.0]
    }
}