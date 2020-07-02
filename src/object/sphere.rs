use crate::vec3::*;
use crate::ray::*;
use super::hittable::*;
use super::*;


pub struct Sphere {
    center: Point3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self {
            center, radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord ) -> bool {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction().length_squared();
        let half_b: f32 = oc.dot(&r.direction());
        let c: f32 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f32 = half_b * half_b - a * c;

        if discriminant > 0f32 {
            let root: f32 = discriminant.sqrt();

            let temp: f32 = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
        }
        false
        
    }
}
