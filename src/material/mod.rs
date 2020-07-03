use crate::object::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(
        &mut self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct MaterialMock;
impl Material for MaterialMock {
    fn scatter(&mut self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }
}

pub mod lambertian;
pub mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;
