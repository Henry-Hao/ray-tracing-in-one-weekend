use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    // pub mat_ptr: Option<Rc<dyn Material>>,
    pub mat_ptr: Rc<RefCell<dyn Material>>,
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: Rc::new(RefCell::new(MaterialMock)),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0f32;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

pub mod hittable;
pub mod hittable_list;
pub mod sphere;

pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use sphere::Sphere;
