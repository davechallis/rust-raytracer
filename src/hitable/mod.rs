use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::bvh::AABB;

mod sphere;
pub use sphere::Sphere;

mod moving_sphere;
pub use moving_sphere::MovingSphere;

mod rectangle;
pub use rectangle::Rectangle;

mod cuboid;
pub use cuboid::Cuboid;

mod constant_medium;
pub use constant_medium::ConstantMedium;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub u: f32,
    pub v: f32,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        Self::new_with_uv(t, point, normal, material, 0.0, 0.0)
    }

    pub fn new_with_uv(t: f32, point: Vec3, normal: Vec3, material: &'a dyn Material, u: f32, v: f32) -> Self {
        Self { t, point, normal, material, u, v }
    }
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

pub struct FlipNormals<T> {
    hitable: T,
}

impl<T: Hitable + Send + Sync> FlipNormals<T> {
    pub fn new(hitable: T) -> Self {
        Self { hitable }
    }
}

impl<T: Hitable + Send + Sync> Hitable for FlipNormals<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hitable.hit(r, t_min, t_max) {
            Some(mut hit_rec) => {
                hit_rec.normal = -hit_rec.normal;
                Some(hit_rec)
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1)
    }
}


pub struct Translate<T> {
    hitable: T,
    offset: Vec3,
}

impl<T: Hitable + Send + Sync> Translate<T> {
    pub fn new(hitable: T, offset: Vec3) -> Self {
        Self { hitable, offset }
    }
}

impl<T: Hitable + Send + Sync> Hitable for Translate<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new_at_time(r.origin() - &self.offset, r.direction().clone(), r.time());
        match self.hitable.hit(&moved_ray, t_min, t_max) {
            Some(mut hit_rec) => {
                hit_rec.point += &self.offset;
                Some(hit_rec)
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.hitable.bounding_box(t0, t1) {
            Some(bbox) => Some(AABB::new(bbox.min() + &self.offset, bbox.max() + &self.offset)),
            None => None,
        }
    }
}

#[derive(Clone)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn idx(&self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

pub struct Rotate<T> {
    hitable: T,
    axis: Axis,
    sin_theta: f32,
    cos_theta: f32,
}

impl<T: Hitable + Send + Sync> Rotate<T> {
    pub fn new_x(hitable: T, angle: f32) -> Self {
        Self::new(hitable, angle, Axis::X)
    }

    pub fn new_y(hitable: T, angle: f32) -> Self {
        Self::new(hitable, angle, Axis::Y)
    }

    pub fn new_z(hitable: T, angle: f32) -> Self {
        Self::new(hitable, angle, Axis::Z)
    }

    fn new(hitable: T, angle: f32, axis: Axis) -> Self {
        let radians = (std::f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        Self { hitable, axis, sin_theta, cos_theta }
    }
}

impl<T: Hitable + Send + Sync> Hitable for Rotate<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let cos_t = self.cos_theta;
        let sin_t = self.sin_theta;

        let (a_idx, b_idx) = match self.axis {
            Axis::X => (1, 2),
            Axis::Y => (0, 2),
            Axis::Z => (0, 1),
        };

        let rotated_ray = {
            let mut origin = r.origin().clone();
            let mut direction = r.direction().clone();
            origin[a_idx] = cos_t * r.origin()[a_idx] - sin_t * r.origin()[b_idx];
            origin[b_idx] = sin_t * r.origin()[a_idx] + cos_t * r.origin()[b_idx];
            direction[a_idx] = cos_t * r.direction()[a_idx] - sin_t * r.direction()[b_idx];
            direction[b_idx] = sin_t * r.direction()[a_idx] + cos_t * r.direction()[b_idx];
            Ray::new_at_time(origin, direction, r.time())
        };

        match self.hitable.hit(&rotated_ray, t_min, t_max) {
            Some(mut hit_rec) => {
                let p = &hit_rec.point.clone();
                let n = &hit_rec.normal.clone();
                hit_rec.point[a_idx] = cos_t * p[a_idx] + sin_t * p[b_idx];
                hit_rec.point[b_idx] = -sin_t * p[a_idx] + cos_t * p[b_idx];
                hit_rec.normal[a_idx] = cos_t * n[a_idx] + sin_t * n[b_idx];
                hit_rec.normal[b_idx] = -sin_t * n[a_idx] + cos_t * n[b_idx];
                Some(hit_rec)
            },
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let bbox = match self.hitable.bounding_box(t0, t1) {
            Some(bbox) => bbox,
            None => return None,
        };

        let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);

        for i in 0..2 {
            let i = i as f32;
            let x = i * bbox.max()[0] + (1.0 - i) * bbox.min()[0];

            for j in 0..2 {
                let j = j as f32;
                let y = j * bbox.max()[1] + (1.0 - j) * bbox.min()[1];

                for k in 0..2 {
                    let k = k as f32;
                    let z = k * bbox.max()[2] + (1.0 - k) * bbox.min()[2];

                    let tester = match self.axis {
                        Axis::X => {
                            let new_y = self.cos_theta * y + self.sin_theta * z;
                            let new_z = -self.sin_theta * y + self.cos_theta * z;
                            Vec3::new(x, new_y, new_z)
                        },
                        Axis::Y => {
                            let new_x = self.cos_theta * x + self.sin_theta * z;
                            let new_z = -self.sin_theta * x + self.cos_theta * z;
                            Vec3::new(new_x, y, new_z)
                        },
                        Axis::Z => {
                            let new_x = self.cos_theta * x + self.sin_theta * y;
                            let new_y = -self.sin_theta * x + self.cos_theta * y;
                            Vec3::new(new_x, new_y, z)
                        },
                    };

                    for c in 0..3 {
                        if tester[c] > max[c] {
                            max[c] = tester[c];
                        }

                        if tester[c] < min[c] {
                            min[c] = tester[c];
                        }
                    }
                }
            }
        }

        Some(AABB::new(min, max))
    }
}
