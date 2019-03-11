use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils;
use crate::hitable::HitRecord;
use crate::texture::Texture;
use super::Material;

#[derive(Clone)]
pub struct Isotropic<T: Texture + Clone> {
    albedo: T,
}

impl<T: Texture + Clone> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture + Clone> Material for Isotropic<T> {
    // pick uniform random direction for scattering
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scattered_ray = Ray::new_at_time(hit_rec.point.clone(), utils::random_in_unit_sphere(), hit_rec.t);
        let attenuation = self.albedo.value(hit_rec.u, hit_rec.v, &hit_rec.point);
        Some((attenuation, scattered_ray))
    }
}
