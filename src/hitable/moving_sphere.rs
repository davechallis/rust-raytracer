use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{HitRecord, Hitable};

// could also be implemented in Sphere, with constant center between times

/// Sphere that moves from center0 to center1 between time0 and time1
pub struct MovingSphere<M: Material> {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: M) -> Self {
        MovingSphere { center0, center1, time0, time1, radius, material }
    }

    pub fn surface_normal(&self, p: &Vec3, time: f32) -> Vec3 {
        (p - self.center(time)) / self.radius
    }

    fn center(&self, time: f32) -> Vec3 {
        &self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (&self.center1 - &self.center0)
    }
}

impl<M: Material> Hitable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let time = r.time();
        let oc = r.origin() - self.center(time); // vector from ray source to sphere center
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let normal = self.surface_normal(&p, time);
                return Some(HitRecord { t, p, normal, material: &self.material });
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let p = r.point_at_parameter(t);
                let normal = self.surface_normal(&p, time);
                return Some(HitRecord { t, p, normal, material: &self.material });
            }
        }

        None
    }
}
