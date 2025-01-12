use crate::hittable::Hittable;
use crate::Vec3;
use crate::Ray;

struct Camera;

impl Camera {
    pub fn render<T: Hittable>(world: &T) {

    }

    fn initialize() {

    }

    fn ray_color<T: Hittable>(r: Ray, world: &T) -> Vec3 {
        
    }
}