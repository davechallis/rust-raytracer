use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::bvh::AABB;

mod sphere;
pub use sphere::Sphere;

mod moving_sphere;
pub use moving_sphere::MovingSphere;

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        None
    }
}

impl Hitable for Vec<Box<dyn Hitable + Send + Sync>> {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let first_box = match self[0].bounding_box(t0, t1) {
            Some(bounding_box) => bounding_box,
            None => return None,
        };

        let mut surrounding_box = first_box.clone();

        for hitable in self[1..].iter() {
            match hitable.bounding_box(t0, t1) {
                Some(bounding_box) => {
                    surrounding_box = AABB::surrounding_box(&surrounding_box, &bounding_box);
                },
                None => return None,
            };
        }

        Some(surrounding_box)
    }
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.is_empty() {
            return None;
        }

        let first_box = match self[0].bounding_box(t0, t1) {
            Some(bounding_box) => bounding_box,
            None => return None,
        };

        let mut surrounding_box = first_box.clone();

        for hitable in self[1..].iter() {
            match hitable.bounding_box(t0, t1) {
                Some(bounding_box) => {
                    surrounding_box = AABB::surrounding_box(&surrounding_box, &bounding_box);
                },
                None => return None,
            };
        }

        Some(surrounding_box)
    }
}
