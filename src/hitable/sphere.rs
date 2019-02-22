use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{HitRecord, Hitable};
use crate::bvh::AABB;

#[derive(Clone)]
pub struct Sphere<'a> {
    center: Vec3,
    radius: f32,
    material: &'a dyn Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Material) -> Self {
        Sphere { center, radius, material }
    }

    pub fn surface_normal(&self, p: &Vec3) -> Vec3 {
        (p - &self.center) / self.radius
    }

    fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let point = (point - &self.center) / self.radius;
        let phi = point[2].atan2(point[0]);
        let theta = point[1].asin();
        let pi = std::f32::consts::PI;
        let u = 1.0 - (phi + pi) / (2.0 * pi);
        let v = (theta + pi / 2.0) / pi;
        (u, v)
    }
}

impl<'a> Hitable for Sphere<'a> {
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
                let (u, v) = self.get_uv(&point);
                return Some(HitRecord::new_with_uv(t, point, normal, self.material, u, v));
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point = r.point_at_parameter(t);
                let normal = self.surface_normal(&point);
                let (u, v) = self.get_uv(&point);
                return Some(HitRecord::new_with_uv(t, point, normal, self.material, u, v));
            }
        }

        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(&self.center - &r, &self.center + &r))
    }
}
