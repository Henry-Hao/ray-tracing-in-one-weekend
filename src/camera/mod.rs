use crate::ray::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta: f32 = degree_to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let origin: Point3 = lookfrom;
        let horizonal: Point3 = focus_dist * viewport_width * u;
        let vertical: Point3 = focus_dist * viewport_height * v;
        Self {
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
            origin,
            horizonal,
            vertical,
            lower_left_corner: origin - horizonal / 2.0 - vertical / 2.0 - w * focus_dist,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizonal + v * self.vertical - self.origin - offset,
        )
    }
}
