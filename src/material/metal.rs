use super::*;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Self {
        Self { albedo: a }
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
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}
