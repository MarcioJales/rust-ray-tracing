use std::fs::File;
use std::io::Write;

mod vec3;

#[macro_use]
extern crate assert_float_eq;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut f = File::create("image.ppm").unwrap();

    writeln!(f, "P3").unwrap();
    writeln!(f, "{image_width} {image_height}").unwrap();
    writeln!(f, "255").unwrap();

    let (mut j, mut i) = (0, 0);

    while j < image_height {
        i = 0;
        println!("\rScanlines remaining: {}", image_height - j);
        while i < image_width {
            let r : f64 = i as f64 / (image_width - 1) as f64;
            let g : f64 = j as f64 / (image_height - 1) as f64;
            let b : f64 = 0.0;

            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;

            writeln!(f, "{} {} {}", ir as i64, ig as i64, ib as i64).unwrap();

            i += 1;
        }
        j += 1;
    }

    println!("\rDone.                 ");
}
