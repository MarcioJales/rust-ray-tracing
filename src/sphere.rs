use std::rc::Rc;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::Vec3;
use crate::Ray;
use crate::hittable::HitRecord;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<dyn Material>
}

impl Hittable for Sphere {
    /* 
    ** The sphere equation is quadritic on the ray variable "t"
    ** Based on that, the a, b and c from Bhaskara are
    ** a = ray_direction * ray_direction
    ** b = -2 * ray_direction * (sphere_center - ray_origin)
    ** c = (sphere_center - ray_origin) * (sphere_center - ray_origin) - sphere_radius^2
    */

    /* 
    ** Section 6.2 proposes a quick simplification of the code that I didn't do
    */
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord ) -> bool {
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
        if !ray_t.surrounds(root) {
            root = (-b + discriminant.sqrt()) / (2.0 * a);
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(self.material.clone());

        return true;
    }
}