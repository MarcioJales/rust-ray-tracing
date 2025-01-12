use crate::{Hittable, HitRecord};
use crate::{Interval, INFINITY};
use crate::Vec3;
use crate::Ray;

struct Camera;

impl Camera {
    pub fn render<T: Hittable>(world: &T) {

    }

    fn initialize() {

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
}