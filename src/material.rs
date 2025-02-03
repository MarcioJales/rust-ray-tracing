use crate::Ray;
use crate::HitRecord;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>{
        return None;
    }
}

pub struct Lambertian {
    albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit_record.normal + Vec3::random_unit();
        let scattered = Ray {
            orig: hit_record.point,
            dir: scatter_direction
        };
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}