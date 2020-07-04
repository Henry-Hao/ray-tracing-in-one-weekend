use super::rtweekend::*;
use std::ops::{AddAssign, DivAssign, Index, MulAssign, Neg, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn random_with_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_double_with_range(min, max),
            random_double_with_range(min, max),
            random_double_with_range(min, max),
        )
    }

    pub fn random() -> Vec3 {
        Vec3::random_with_range(0.0, 1.0)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p: Vec3 = Vec3::random();
            if p.length_squared() <= 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = random_double_with_range(0.0, 2.0 * PI);
        let z = random_double_with_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Color::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn reflect(u: &Vec3, n: &Vec3) -> Vec3 {
        *u - 2.0 * u.dot(&n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = n.dot(&(-(*uv)));
        let r_out_parallel: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_perp: Vec3 = -(1.0 - r_out_parallel.length_squared()).sqrt() * *n;
        r_out_parallel + r_out_perp
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &0f32,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub mod color;
pub mod vector;

/**************************
 *   Unit Test
 **************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        let (x, y, z) = (1f32, 2f32, 3f32);
        let v: Vec3 = Vec3::new(x, y, z);
        assert_eq!((-v).x, -x);
        assert_eq!((-v).y, -y);
        assert_eq!((-v).z, -z);
    }

    #[test]
    fn test_index() {
        let (x, y, z) = (1f32, 2f32, 3f32);
        let v: Vec3 = Vec3::new(x, y, z);
        assert_eq!(v[0], x);
        assert_eq!(v[1], y);
        assert_eq!(v[2], z);
        assert_eq!(v[3], 0f32);
    }

    #[test]
    fn test_add_assign() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let mut v1: Vec3 = Vec3::new(x1, y1, z1);
        let v2: Vec3 = Vec3::new(x2, y2, z2);
        let v3: Vec3 = Vec3::new(x1 + x2, y1 + y2, z1 + z2);
        v1 += v2;
        assert_eq!(v3.x, v1.x);
        assert_eq!(v3.y, v1.y);
        assert_eq!(v3.z, v1.z);
    }

    #[test]
    fn test_sub_assign() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let mut v1: Vec3 = Vec3::new(x1, y1, z1);
        let v2: Vec3 = Vec3::new(x2, y2, z2);
        let v3: Vec3 = Vec3::new(x1 - x2, y1 - y2, z1 - z2);
        v1 -= v2;
        assert_eq!(v3.x, v1.x);
        assert_eq!(v3.y, v1.y);
        assert_eq!(v3.z, v1.z);
    }

    #[test]
    fn test_mul_assign() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let mut v1: Vec3 = Vec3::new(x1, y1, z1);
        let v2: Vec3 = Vec3::new(x2, y2, z2);
        let v3: Vec3 = Vec3::new(x1 * x2, y1 * y2, z1 * z2);
        v1 *= v2;
        assert_eq!(v3.x, v1.x);
        assert_eq!(v3.y, v1.y);
        assert_eq!(v3.z, v1.z);
        v1 *= 10f32;
        assert_eq!(v3.x * 10f32, v1.x);
        assert_eq!(v3.y * 10f32, v1.y);
        assert_eq!(v3.z * 10f32, v1.z);
    }

    #[test]
    fn test_div_assign() {
        let (x1, y1, z1) = (1f32, 2f32, 3f32);
        let (x2, y2, z2) = (4f32, 5f32, 6f32);
        let mut v1: Vec3 = Vec3::new(x1, y1, z1);
        let v2: Vec3 = Vec3::new(x2, y2, z2);
        let v3: Vec3 = Vec3::new(x1 / x2, y1 / y2, z1 / z2);
        v1 /= v2;
        assert_eq!(v3.x, v1.x);
        assert_eq!(v3.y, v1.y);
        assert_eq!(v3.z, v1.z);
    }

    #[test]
    fn test_length_squared() {
        let (x1, y1, z1) = (3f32, 4f32, 6f32);
        let v1: Point3 = Point3::new(x1, y1, z1);
        // assert_eq!(v1.length_squared(), (3*3+4*4+6*6) as f32);
        assert_eq!(v1.length_squared(), 61f32);
    }

    #[test]
    fn test_length() {
        let (x1, y1, z1) = (3f32, 4f32, 5f32);
        let v1: Point3 = Point3::new(x1, y1, z1);
        assert_eq!(v1.length(), 7.071068f32);
    }
}
