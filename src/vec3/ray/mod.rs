use crate::vec3::*;

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

    pub fn gradient_color(&self) -> Color {
        if self.hits_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0,0.0,0.0);
        }
        let unit_direction: Point3 = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn hits_sphere(&self, center: &Point3, r: f32) -> bool {
        let origin_center: Point3 = self.origin() - *center;
        let a: f32 = self.direction().dot(&self.direction());
        let b: f32 = 2f32 * self.direction().dot(&origin_center);
        let c: f32 = origin_center.dot(&origin_center) - r * r;
        b * b - 4f32 * a * c >= 0f32
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

    #[test]
    fn test_hit_sphere_once() {
        let center: Point3 = Point3::new(0f32,0f32,0f32);
        let r: f32 = 1f32;
        let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
        let direction: Point3 = Point3::new(2f32.sqrt(),-(2f32.sqrt()),0f32);
        let ray: Ray = Ray::new(origin, direction);
        assert!(ray.hits_sphere(&center, r))
    }

    #[test]
    fn test_hit_sphere_twice() {
        let center: Point3 = Point3::new(0f32,0f32,0f32);
        let r: f32 = 1f32;
        let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
        let direction: Point3 = Point3::new(1f32,-(2f32.sqrt()),0f32);
        let ray: Ray = Ray::new(origin, direction);
        assert!(ray.hits_sphere(&center, r))
    }

    #[test]
    fn test_not_hit_sphere() {
        let center: Point3 = Point3::new(0f32,0f32,0f32);
        let r: f32 = 1f32;
        let origin: Point3 = Point3::new(0f32, 2f32.sqrt(), 0f32);
        let direction: Point3 = Point3::new(2f32,-(2f32.sqrt()),0f32);
        let ray: Ray = Ray::new(origin, direction);
        assert!(!ray.hits_sphere(&center, r))
    }
}
