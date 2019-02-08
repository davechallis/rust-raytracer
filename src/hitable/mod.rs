use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

mod sphere;
pub use sphere::Sphere;

mod moving_sphere;
pub use moving_sphere::MovingSphere;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hitable for [Box<dyn Hitable + Send + Sync>] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_t = t_max;
        for hitable in self {
            if let Some(hit) = hitable.hit(r, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}
