use crate::Ray;
use crate::HitRecord;
use crate::Vec3;
use crate::random::random;

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

impl Dielectric {
    /* Schlink's approximation for reflectance 
    ** Reflectance is a probability that the ray will be reflected, instead of refracted
    ** The higher the reflectance, the more likely the ray will be reflected
    */
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut refraction_i = self.refraction_index;

        if hit_record.front_face {
            refraction_i = 1.0/self.refraction_index;
        }

        let cos_in = (-ray_in.direction().dot(hit_record.normal)).min(1.0);
        let sin_in = (1.0 - cos_in * cos_in).sqrt();

        /* The ray is reflected instead of refracted in 2 cases:
        ** 1. The ray is coming from a material with a higher refraction index than the current material and the angle is too steep
        ** 2. Disregarding the material, reflectance is higher than a random number
        */
        let direction;
        if refraction_i * sin_in > 1.0 || Dielectric::reflectance(cos_in, refraction_i) > random() {
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