use crate::vec3::*;
use crate::ray::*;
use std::marker::Copy;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    
    pub fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0f32;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal
        }
    }
}

pub mod hittable;
pub mod hittable_list;
pub mod sphere;


pub use sphere::Sphere;
pub use hittable_list::HittableList;
