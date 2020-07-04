use super::*;

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(
        &mut self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let etai_over_etat: f32 = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction: Vec3 = r_in.direction().unit_vector();
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let mut cos_theta: f32 = (-unit_direction).dot(&rec.normal);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob: f32 = schlick(cos_theta, etai_over_etat);
        if etai_over_etat * sin_theta > 1.0 || random_double() < reflect_prob {
            let reflected: Vec3 = Vec3::reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let refracted: Vec3 = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
