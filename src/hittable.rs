use crate::Vec3;
use crate::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: HitRecord ) -> bool;
}