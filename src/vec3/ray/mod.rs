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
        let unit_direction: Point3 = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0-t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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
}
