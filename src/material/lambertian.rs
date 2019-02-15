use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils;
use crate::hitable::HitRecord;
use crate::texture::Texture;
use super::Material;

#[derive(Clone)]
pub struct Lambertian<T: Texture + Clone> {
    albedo: T,
}

impl<T: Texture + Clone> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Lambertian { albedo }
    }
}

impl<T: Texture + Clone> Material for Lambertian<T> {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // get random scatter direction from unit sphere
        let target = &hit_rec.point + &hit_rec.normal + utils::random_in_unit_sphere();

        // new ray from hit point
        let scattered_ray = Ray::new_at_time(hit_rec.point.clone(), &target - &hit_rec.point, ray_in.time());
        let attenuation = self.albedo.value(0.0, 0.0, &hit_rec.point);

        Some((attenuation, scattered_ray))
    }
}
