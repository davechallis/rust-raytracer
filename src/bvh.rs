use std::cmp::Ordering;
use rand::Rng;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::{HitRecord, Hitable};

#[derive(Clone, Debug)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    // core logic
    pub fn _hit_orig(&self, ray_in: &Ray, tmin: f32, tmax: f32) -> bool {
        let ray_origin = ray_in.origin();
        let ray_direction = ray_in.direction();
        for axis in 0..3 {
            let axis_min = self.min[axis] - ray_origin[axis] / ray_direction[axis];
            let axis_max = self.max[axis] - ray_origin[axis] / ray_direction[axis];
            let t0 = ffmin(axis_min, axis_max);
            let t1 = ffmax(axis_min, axis_max);
            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    // more efficient implementation
    pub fn hit(&self, ray_in: &Ray, tmin: f32, tmax: f32) -> bool {
        let ray_direction = ray_in.direction();
        let ray_origin = ray_in.origin();
        for axis in 0..3 {
            let inv_d = 1.0 / ray_direction[axis];
            let origin = ray_origin[axis];
            let mut t0 = (self.min[axis] - origin) * inv_d;
            let mut t1 = (self.max[axis] - origin) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin {
                t0
            } else {
                tmin
            };

            let tmax = if t1 < tmax {
                t1
            } else {
                tmax
            };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(box0.min[0].min(box1.min[0]),
                              box0.min[1].min(box1.min[1]),
                              box0.min[2].min(box1.min[2]));
        let big = Vec3::new(box0.max[0].max(box1.max[0]),
                            box0.max[1].max(box1.max[1]),
                            box0.max[2].max(box1.max[2]));
        AABB::new(small, big)
    }
}

pub struct BvhNode {
    left: Option<Box<dyn Hitable + Send + Sync>>,
    right: Option<Box<dyn Hitable + Send + Sync>>,
    bounding_box: AABB,
}

impl BvhNode {
    // TODO: lots of optimisation here
    // 1. randomly choose an axis
    // 2. sort primitives
    // 3. put half in each subtree
    pub fn from_vec(mut hitables: Vec<Box<dyn Hitable + Send + Sync>>, time0: f32, time1: f32) -> Self {
        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0, 3);

        hitables.sort_by(|a, b| {
            let box_a = a.bounding_box(0.0, 0.0).expect("no bounding box in BvhNode::new constructor");
            let box_b = b.bounding_box(0.0, 0.0).expect("no bounding box in BvhNode::new constructor");
            box_compare(&box_a, &box_b, axis)
        });

        let size = hitables.len();
        let (left, right): (Option<Box<dyn Hitable + Send + Sync>>, Option<Box<dyn Hitable + Send + Sync>>) = match size {
            1 => {
                let left_node = Some(hitables.remove(0));
                let right_node = None;
                (left_node, right_node)
            },
            2 => {
                let left_node = Some(hitables.remove(0));
                let right_node = Some(hitables.remove(0));
                (left_node, right_node)
            },
            size => {
                let mid = size / 2;
                let right = hitables.split_off(mid);
                let left_node: Option<Box<dyn Hitable + Send + Sync>> = Some(Box::new(BvhNode::from_vec(hitables, time0, time1)));
                let right_node: Option<Box<dyn Hitable + Send + Sync>> = Some(Box::new(BvhNode::from_vec(right, time0, time1)));
                (left_node, right_node)
            },
        };

        let box_left = left.as_ref()
            .expect("left node should always be Some")
            .bounding_box(time0, time1)
            .expect("no bounding box in BvhNode::new constructor");

        let box_right = match right {
            Some(ref right) => right.bounding_box(time0, time1).expect("no bounding box in BvhNode::new constructor"),
            None => box_left.clone(),
        };

        let bounding_box = AABB::surrounding_box(&box_left, &box_right);

        Self { left, right, bounding_box }
    }
}

fn box_compare(a: &AABB, b: &AABB, axis: usize) -> Ordering {
    a.min[axis].partial_cmp(&b.min[axis]).expect("box comparison should succeed here")
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                 match (left.hit(r, t_min, t_max), right.hit(r, t_min, t_max)) {
                    (Some(left_hit_rec), Some(right_hit_rec)) => {
                        if left_hit_rec.t < right_hit_rec.t {
                            Some(left_hit_rec)
                        } else {
                            Some(right_hit_rec)
                        }
                    },
                    (Some(left_hit_rec), None) => Some(left_hit_rec),
                    (None, Some(right_hit_rec)) => Some(right_hit_rec),
                    (None, None) => None,
                }
            },
            (Some(left), None) => left.hit(r, t_min, t_max),
            (None, Some(right)) => right.hit(r, t_min, t_max),
            (None, None) => None,
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}


fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
