use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{HitRecord, Hitable};
use crate::bvh::AABB;

enum Plane {
    XY,
    YZ,
    XZ,
}

// rectangle on xy plane, between two x and y values (z is fixed at z = k)
#[derive(Clone)]
pub struct Rectangle<M: Material> {
    a_bound: (f32, f32),
    b_bound: (f32, f32),
    a_idx: usize,
    b_idx: usize,
    k_idx: usize,
    plane_normal: Vec3,
    k: f32,
    material: M,
}

impl<M: Material> Rectangle<M> {
    pub fn new_xy(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, material: M) -> Self {
        Self::new(a_bound, b_bound, Plane::XY, k, material)
    }

    pub fn new_yz(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, material: M) -> Self {
        Self::new(a_bound, b_bound, Plane::YZ, k, material)
    }

    pub fn new_xz(a_bound: (f32, f32), b_bound: (f32, f32), k: f32, material: M) -> Self {
        Self::new(a_bound, b_bound, Plane::XZ, k, material)
    }

    fn new(a_bound: (f32, f32), b_bound: (f32, f32), plane: Plane, k: f32, material: M) -> Self {
        let (a_idx, b_idx, k_idx, plane_normal) = match plane {
            Plane::XY => (0, 1, 2, Vec3::new(0.0, 0.0, 1.0)),
            Plane::YZ => (1, 2, 0, Vec3::new(1.0, 0.0, 0.0)),
            Plane::XZ => (0, 2, 1, Vec3::new(0.0, 1.0, 0.0)),
        };
        Self { a_bound, b_bound, a_idx, b_idx, k_idx, plane_normal, k, material }
    }
}

impl<M: Material> Hitable for Rectangle<M> {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let origin = r.origin();
        let direction = r.direction();

        let t = (self.k - origin[self.k_idx]) / direction[self.k_idx];
        if t < t0 || t > t1 {
            return None;
        }

        let a = origin[self.a_idx] + t * direction[self.a_idx];
        if a < self.a_bound.0 || a > self.a_bound.1 {
            return None;
        }

        let b = origin[self.b_idx] + t * direction[self.b_idx];
        if b < self.b_bound.0 || b > self.b_bound.1 {
            return None;
        }

        let hit_rec = HitRecord::new_with_uv(
            t,
            r.point_at_parameter(t),
            self.plane_normal.clone(),
            &self.material,
            (a - self.a_bound.0) / (self.a_bound.1 - self.a_bound.0),
            (b - self.b_bound.0) / (self.b_bound.1 - self.b_bound.0),
        );
        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(self.a_bound.0, self.b_bound.0, self.k - 0.0001);
        let max = Vec3::new(self.a_bound.1, self.b_bound.1, self.k + 0.0001);
        Some(AABB::new(min, max))
    }
}
