use rand::prelude::*;
use crate::vec3::Vec3;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::bvh::AABB;
use crate::texture::Texture;
use crate::material::Isotropic;

#[derive(Clone)]
pub struct ConstantMedium<H, T: Texture + Clone> {
    boundary: H,
    density: f32,
    phase_function: Isotropic<T>,
}

impl<H: Hitable + Send + Sync, T: Texture + Clone> ConstantMedium<H, T> {
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        Self { boundary, density, phase_function: Isotropic::new(texture) }
    }
}

// careful boundary logic needed for ray origins inside the volume - common in clouds where bouncing occurs often
impl<H: Hitable + Send + Sync, T: Texture + Clone> Hitable for ConstantMedium<H, T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        if let Some(hit_rec1) = self.boundary.hit(r, std::f32::MIN, std::f32::MAX) {
            if let Some(hit_rec2) = self.boundary.hit(r, hit_rec1.t + 0.0001, std::f32::MAX) {
                let mut hr1 = hit_rec1.clone();
                let mut hr2 = hit_rec2.clone();

                if hr1.t < t_min {
                    hr1.t = t_min;
                }

                if hr2.t > t_max {
                    hr2.t = t_max;
                }

                if hr1.t >= hr2.t {
                    return None;
                }

                if hr1.t < 0.0 {
                    hr1.t = 0.0;
                }

                let mut rng = rand::thread_rng();

                let distance_inside_boundary = (hr2.t - hr1.t) * r.direction().length();
                let hit_distance = -(1.0 / self.density) * rng.gen::<f32>().ln();

                if hit_distance < distance_inside_boundary {
                    let t = hr1.t + hit_distance / r.direction().length();
                    let p = r.point_at_parameter(t);
                    let n = Vec3::new(1.0, 0.0, 0.0); // arbitrary, does nothing
                    return Some(HitRecord::new(t, p, n, &self.phase_function))
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
