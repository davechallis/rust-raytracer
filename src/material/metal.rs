use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use super::Material;
use crate::utils;

#[derive(Clone)]
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
        let unit_dir = r.direction().to_unit_vector();
        let reflected = utils::reflect(&unit_dir, &hit_rec.normal);

        // new ray from hit point
        let scattered = Ray::new(hit_rec.p.clone(), reflected + self.fuzz * utils::random_in_unit_sphere());
        let attenuation = self.albedo.clone();

        let x = scattered.direction().dot(&hit_rec.normal);
        if x > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

