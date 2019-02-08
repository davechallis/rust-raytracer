use rand::prelude::*;
use crate::utils;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::HitRecord;
use crate::material::Material;

pub struct Dielectric {
    reflective_index: f32,
}

impl Dielectric {
    pub fn new(reflective_index: f32) -> Self {
        Dielectric { reflective_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = utils::reflect(&r.direction(), &hit_rec.normal);

        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let d = r.direction().dot(&hit_rec.normal);
        let (outward_normal, ni_over_nt, cosine) = if d > 0.0 {
            let cosine = self.reflective_index * d / r.direction().length();
            (-hit_rec.normal.clone(), self.reflective_index, cosine)
        } else {
            let cosine = -d / r.direction().length();
            (hit_rec.normal.clone(), 1.0 / self.reflective_index, cosine)
        };

        let (refracted_ray, reflect_prob) = match utils::refract(&r.direction(), &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = utils::schlick(cosine, self.reflective_index);
                (Some(refracted), reflect_prob)
            },
            None => {
                // TODO: remove
                //scattered_ray = Some(Ray::new(hit_rec.p, reflected));
                let reflect_prob = 1.0;
                (None, reflect_prob)
            },
        };

        let mut rng = rand::thread_rng();
        let scattered_ray = if rng.gen::<f32>() < reflect_prob {
            Ray::new_at_time(hit_rec.p.clone(), reflected, r.time())
        } else {
            Ray::new_at_time(hit_rec.p.clone(), refracted_ray.unwrap(), r.time())
        };

        Some((attenuation, scattered_ray))
    }
}
