use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::hitable::{FlipNormals, HitRecord, Hitable};
use crate::bvh::AABB;
use crate::hitable::rectangle::Rectangle;

pub struct Cuboid {
    p_min: Vec3,
    p_max: Vec3,
    hitables: Vec<Box<dyn Hitable + Send + Sync>>,
}

impl Cuboid {
    pub fn new<M: 'static + Material + Clone>(p_min: Vec3, p_max: Vec3, material: M) -> Self {
        let x0 = p_min[0];
        let x1 = p_max[0];

        let y0 = p_min[1];
        let y1 = p_max[1];

        let z0 = p_min[2];
        let z1 = p_max[2];

        let hitables: Vec<Box<dyn Hitable + Send + Sync>> = vec![
            Box::new(Rectangle::new_xy((x0, x1), (y0, y1), z1, material.clone())),
            Box::new(FlipNormals::new(Rectangle::new_xy((x0, x1), (y0, y1), z0, material.clone()))),

            Box::new(Rectangle::new_xz((x0, x1), (z0, z1), y1, material.clone())),
            Box::new(FlipNormals::new(Rectangle::new_xz((x0, x1), (z0, z1), y0, material.clone()))),

            Box::new(Rectangle::new_yz((y0, y1), (z0, z1), x1, material.clone())),
            Box::new(FlipNormals::new(Rectangle::new_yz((y0, y1), (z0, z1), x0, material.clone()))),
        ];

        Self { p_min, p_max, hitables }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitables.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.p_min.clone(), self.p_max.clone()))
    }
}
