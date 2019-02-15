use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{HitRecord, Hitable};
use crate::bvh::AABB;

#[derive(Clone)]
pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Self {
        Sphere { center, radius, material }
    }

    pub fn surface_normal(&self, p: &Vec3) -> Vec3 {
        (p - &self.center) / self.radius
    }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center; // vector from ray source to sphere center
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point = r.point_at_parameter(t);
                let normal = self.surface_normal(&point);
                return Some(HitRecord { t, point, normal, material: &self.material });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point = r.point_at_parameter(t);
                let normal = self.surface_normal(&point);
                return Some(HitRecord { t, point, normal, material: &self.material });
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(&self.center - &r, &self.center + &r))
    }
}
