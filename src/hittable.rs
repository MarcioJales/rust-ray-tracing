use std::rc::Rc;

use crate::Vec3;
use crate::Ray;

#[derive(Clone)]
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
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord ) -> bool;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord ) -> bool {
        let mut temp_rec = HitRecord {
            point: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();
            }
        }

        hit_anything
    }
}