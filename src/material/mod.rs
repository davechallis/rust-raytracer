mod metal;
mod lambertian;
mod dielectric;
mod diffuse_light;
mod isotropic;

pub use self::metal::Metal;
pub use self::lambertian::Lambertian;
pub use self::dielectric::Dielectric;
pub use self::diffuse_light::DiffuseLight;
pub use self::isotropic::Isotropic;

use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::HitRecord;

pub trait Material: Send + Sync {
    // (attenuation, ray)
    fn scatter(&self, r: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)>;

    // default to emitting black
    fn emitted(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        Vec3::zeros()
    }
}
