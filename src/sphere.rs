use crate::hittable::Hittable;
use crate::Vec3;
use crate::Ray;
use crate::hittable::HitRecord;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord ) -> bool {
        let orig_to_center = self.center - ray.origin();
        let a = ray.direction().dot(ray.direction());
        let b = -2.0 * ray.direction().dot(orig_to_center);
        let c = orig_to_center.dot(orig_to_center) - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false;
        } 

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-b - discriminant.sqrt()) / (2.0 * a);
        if root <= ray_tmin || ray_tmax <= root {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = (hit_record.point - self.center) / self.radius;

        return true;
    }
}