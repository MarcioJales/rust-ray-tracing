use crate::Ray;
use crate::HitRecord;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord,  attenuation: &Vec3, scattered: &Ray) -> bool {
        return false;
    }
}