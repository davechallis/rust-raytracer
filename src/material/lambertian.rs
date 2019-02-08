use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utils;
use crate::hitable::HitRecord;
use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // get random scatter direction from unit sphere
        let target = &hit_rec.p + &hit_rec.normal + utils::random_in_unit_sphere();

        // new ray from hit point
        let scattered_ray = Ray::new_at_time(hit_rec.p.clone(), &target - &hit_rec.p, ray_in.time());
        let attenuation = self.albedo.clone();

        Some((attenuation, scattered_ray))
    }
}
