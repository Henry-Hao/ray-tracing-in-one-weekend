use crate::ray::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = degree_to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let origin: Point3 = lookfrom;
        let horizonal: Point3 = viewport_width * u;
        let vertical: Point3 = viewport_height * v;
        Self {
            origin,
            horizonal,
            vertical,
            lower_left_corner: origin
                - horizonal / 2.0
                - vertical / 2.0
                - w
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizonal + v * self.vertical - self.origin,
        )
    }
}
