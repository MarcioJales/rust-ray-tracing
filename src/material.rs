use crate::Ray;
use crate::HitRecord;
use crate::Vec3;

pub struct Material {
}

impl Material {
    pub fn scatter(ray_in: &Ray, hit_record: &HitRecord,  attenuation: &Vec3, scattered: &Ray) -> bool {
        return false;
    }
}