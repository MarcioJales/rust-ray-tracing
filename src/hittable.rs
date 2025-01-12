use std::rc::Rc;

use crate::Vec3;
use crate::Ray;
use crate::interval::Interval;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /* 
    ** The normal vector points against the ray. 
    ** In this case, rays that are inside the sphere must have the normal = -outward_normal 
    */
   pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {

        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if let true = self.front_face {
            self.normal = outward_normal;
        }
        else {
            self.normal = -1.0 * outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord ) -> bool;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord ) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if object.hit(ray, Interval(ray_t.min(), closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }

        hit_anything
    }
}