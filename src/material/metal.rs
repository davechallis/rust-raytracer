use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use super::Material;
use crate::utils;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Metal<T: Texture + Clone> {
    albedo: T,
    fuzz: f32,
}

impl<T: Texture + Clone> Metal<T> {
    pub fn new(albedo: T, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal { albedo, fuzz }
    }
}

impl<T: Texture + Clone> Material for Metal<T> {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let unit_dir = ray_in.direction().to_unit_vector();
        let reflected = utils::reflect(&unit_dir, &hit_rec.normal);

        // new ray from hit point
        let scattered_ray = Ray::new_at_time(hit_rec.point.clone(), reflected + self.fuzz * utils::random_in_unit_sphere(), ray_in.time());

        let x = scattered_ray.direction().dot(&hit_rec.normal);
        if x > 0.0 {
            let attenuation = self.albedo.value(0.0, 0.0, &hit_rec.point);
            Some((attenuation, scattered_ray))
        } else {
            None
        }
    }
}

