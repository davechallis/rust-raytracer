use rand::prelude::*;
use crate::vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    // get random point from unit cube -1 to +1, reject if outside sphere
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)) - 1.0;
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disc() -> Vec3 {
    let mut rng = thread_rng();
    loop {
        let p = 2.0 * Vec3::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}


// ray reflection is v + 2B. N is unit vector, so len of B is v.N. This points in, so negate.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0-dt.powi(2));
    let n = *n;
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

// approximation for reflectivity that varies with angle
pub fn schlick(cosine: f32, reflective_index: f32) -> f32 {
    let r0 = ((1.0 - reflective_index) / (1.0 + reflective_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
