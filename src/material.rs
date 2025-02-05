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
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray {
            orig: hit_record.point,
            dir: scatter_direction
        };
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_direction = ray_in.direction().reflect(&hit_record.normal);

        let reflected = Ray {
            orig: hit_record.point,
            dir: reflected_direction
        };
        let attenuation = self.albedo;

        Some((reflected, attenuation))
    }
}