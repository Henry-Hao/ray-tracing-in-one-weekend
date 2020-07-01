use super::*;
use std::ops::{Add, Div, Mul};

impl Point3 {
    pub fn dot(&self, rhs: &Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

}

impl Add for Point3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for Point3{
    type Output = Self;
    fn mul(self, rhs:f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }

}

impl Mul for Point3 {
    type Output = Point3;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f32> for Point3 {
    type Output = Point3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1f32/rhs)
    }
}


/**************************
 * Unit Test
 **************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let v1: Point3 = Point3 ::new(x1, y1, z1);
        let v2: Point3 = Point3 ::new(x2, y2, z2);
        let v3: Point3 = v1 + v2;
        assert_eq!(v3.x, x1 + x2);
        assert_eq!(v3.y, y1 + y2);
        assert_eq!(v3.z, z1 + z2);
    }

    #[test]
    fn test_mul() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let v1: Point3 = Point3::new(x1, y1, z1);
        let v2: Point3 = Point3::new(x2, y2, z2);
        let v3: Point3 = v1 * v2;
        assert_eq!(v3.x, x1 * x2);
        assert_eq!(v3.y, y1 * y2);
        assert_eq!(v3.z, z1 * z2);
        let v4: Point3 = v1 * 10f32;
        assert_eq!(v4.x, x1*10f32);
        assert_eq!(v4.y, y1*10f32);
        assert_eq!(v4.z, z1*10f32);

    }

    #[test]
    fn test_div() {
        let (x, y, z) = (3f32, 6f32, 9f32);
        let v: Point3 = Point3 ::new(x, y, z);
        let v2: Point3 = v/3f32;
        assert_eq!(v2.x, 1f32);
        assert_eq!(v2.y, 2f32);
        assert_eq!(v2.z, 3f32);
    }

    #[test]
    fn test_dot() {
        let (x1, y1, z1) = (3f32, 4f32, 5f32);
        let (x2, y2, z2) = (6f32, 7f32, 8f32);
        let v1: Point3 = Point3::new(x1, y1, z1);
        let v2: Point3 = Point3::new(x2, y2, z2);
        assert_eq!(v1.dot(&v2), Point3::new(x1*x2, y1*y2, z1*z2));
    }

    #[test]
    fn test_cross() {
        let (x1, y1, z1) = (3f32, 4f32, 5f32);
        let (x2, y2, z2) = (6f32, 7f32, 8f32);
        let v1: Point3 = Point3::new(x1, y1, z1);
        let v2: Point3 = Point3::new(x2, y2, z2);
        assert_eq!(v1.cross(&v2), Point3::new(-3.0, 6.0, -3.0));
    }
}
