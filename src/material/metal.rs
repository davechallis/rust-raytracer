use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use super::Material;
use crate::utils;

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = utils::reflect(&r.direction().to_unit_vector(), &hit_rec.normal);

        // new ray from hit point
        let scattered = Ray::new(hit_rec.p, reflected + self.fuzz * utils::random_in_unit_sphere());
        let attenuation = self.albedo;

        let x = scattered.direction().dot(&hit_rec.normal);
        if x > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

