use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{HitRecord, Hitable};
use crate::bvh::AABB;

// rectangle on xy plane, between two x and y values (z is fixed at z = k)
#[derive(Clone)]
pub struct XYRectangle<M: Material> {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: M,
}

impl<M: Material> XYRectangle<M> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: M) -> Self {
        Self { x0, x1, y0, y1, k, material }
    }
}

impl<M: Material> Hitable for XYRectangle<M> {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let origin = r.origin();
        let direction = r.direction();
        let t = (self.k - origin[2]) / direction[2];
        if t < t0 || t > t1 {
            return None;
        }

        let x = origin[0] + t * direction[0];
        if x < self.x0 || x > self.x1 {
            return None;
        }

        let y = origin[1] + t * direction[1];
        if y < self.y0 || y > self.y1 {
            return None;
        }

        let hit_rec = HitRecord::new_with_uv(
            t,
            r.point_at_parameter(t),
            Vec3::new(0.0, 0.0, 1.0),
            &self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        );
        Some(hit_rec)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(self.x0, self.y0, self.k - 0.0001);
        let max = Vec3::new(self.x1, self.y1, self.k + 0.0001);
        Some(AABB::new(min, max))
    }
}
