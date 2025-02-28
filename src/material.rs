use crate::Ray;
use crate::HitRecord;
use crate::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>{
        return None;
    }
}

pub struct Lambertian {
    pub albedo: Vec3
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
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_direction = ray_in.direction().reflect(&hit_record.normal);
        let fuzz_direction = reflected_direction.unit() + (self.fuzz * Vec3::random_unit());

        if fuzz_direction.dot(hit_record.normal) > 0.0 {
            let reflected = Ray {
                orig: hit_record.point,
                dir: fuzz_direction
            };
            let attenuation = self.albedo;

            Some((reflected, attenuation))
        }
        /* If the fuzz calculation goes inwards the material, we absorb it */
        else {
            return None;
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64
}


impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut refraction_i = self.refraction_index;

        if hit_record.front_face {
            refraction_i = 1.0/self.refraction_index;
        }

        let cos_in = (-ray_in.direction().dot(hit_record.normal)).min(1.0);
        let sin_in = (1.0 - cos_in * cos_in).sqrt();

        /* That is, the ray cannot refract, since the incident angle is too big in relation to the surface normal  */
        let direction;
        if refraction_i * sin_in > 1.0 {
            /* reflect is borrowing the normal, while the refract isn't. It would be nice to make them the same way */
            direction = ray_in.direction().unit().reflect(&hit_record.normal);
        }
        else {
            direction = ray_in.direction().unit().refract(hit_record.normal, refraction_i);
        }

        let scattered = Ray {
            orig: hit_record.point,
            dir: direction
        };

        let attenuation = Vec3(1.0, 1.0, 1.0);

        Some((scattered, attenuation))
    }
}