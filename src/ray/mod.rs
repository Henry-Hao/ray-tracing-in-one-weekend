use crate::vec3::*;
use crate::object::hittable::Hittable;
use crate::object::*;
use crate::rtweekend::*;

pub struct Ray {
    origin: Point3,
    direction: Point3
}


impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn ray_color<T: Hittable>(&self, world: &T, depth: u16) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec:HitRecord = HitRecord::default();
        if world.hit(self, 0.001, INFINITY, &mut rec) {
            // let target: Point3 = rec.p + rec.normal + Vec3::random_unit_vector();
            let target: Point3 = rec.p + Color::random_in_hemisphere(&rec.normal);
            let ray: Ray = Ray::new(rec.p, target - rec.p);
            return 0.5 * (ray.ray_color(world, depth-1));
        }

        let unit_direction: Vec3 = self.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


/**********
 * Unit Test
 ***********/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_origin() {
        let (x,y,z) = (1f32, 2f32, 3f32);
        let origin: Point3 = Point3::new(x, y, z);
        let direction: Point3 = Point3::new(x, y, z);
        let r: Ray = Ray::new(origin, direction);
        assert_eq!(r.origin(), Point3::new(x, y, z));
    }

    #[test]
    fn test_direction() {
        let (x,y,z) = (1f32, 2f32, 3f32);
        let origin: Point3 = Point3::new(x, y, z);
        let direction: Point3 = Point3::new(x, y, z);
        let r: Ray = Ray::new(origin, direction);
        assert_eq!(r.direction(), Point3::new(x, y, z));
    }

    #[test]
    fn test_at() {
        let (x,y,z) = (1f32, 2f32, 3f32);
        let origin: Point3 = Point3::new(x, y, z);
        let direction: Point3 = Point3::new(x, y, z);
        let r: Ray = Ray::new(origin, direction);
        assert_eq!(r.at(3f32), Point3::new(4f32, 8f32, 12f32));
    }

    // #[test]
    // fn test_hit_sphere_once() {
    //     let center: Point3 = Point3::new(0f32,0f32,0f32);
    //     let r: f32 = 1f32;
    //     let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
    //     let direction: Point3 = Point3::new(2f32.sqrt(),-(2f32.sqrt()),0f32);
    //     let ray: Ray = Ray::new(origin, direction);
    //     assert_eq!(ray.hits_sphere(&center, r),0.5)
    // }
    //
    // #[test]
    // fn test_hit_sphere_twice() {
    //     let center: Point3 = Point3::new(0f32,0f32,0f32);
    //     let r: f32 = 1f32;
    //     let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
    //     let direction: Point3 = Point3::new(1f32,-(2f32.sqrt()),0f32);
    //     let ray: Ray = Ray::new(origin, direction);
    //     assert_eq!(ray.hits_sphere(&center, r), 1.0)
    // }
    //
    #[test]
    fn test_not_hit_sphere() {
        let center: Point3 = Point3::new(0f32,0f32,0f32);
        let r: f32 = 1f32;
        let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
        let direction: Point3 = Point3::new(2f32,-(2f32.sqrt()),0f32);
        let ray: Ray = Ray::new(origin, direction);
        assert_eq!(ray.hits_sphere(&center, r), -1.0)
    }
}
