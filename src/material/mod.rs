mod metal;
mod lambertian;
mod dielectric;

pub use self::metal::Metal;
pub use self::lambertian::Lambertian;
pub use self::dielectric::Dielectric;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::HitRecord;

pub trait Material {
    // (attenuation, ray)
    fn scatter(&self, r: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

