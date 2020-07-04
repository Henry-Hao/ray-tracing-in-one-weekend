use super::*;

pub struct Metal {
    albedo: Color,
    fuzz: f32
}

impl Metal {
    pub fn new(a: Color, f: f32) -> Self {
        Self {
            albedo: a,
            fuzz: match f < 1.0 {
                true => f,
                false => 1.0
            }
        }
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
