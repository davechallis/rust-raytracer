use rand::prelude::*;
use crate::vec3::Vec3;

const SIZE: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranfloat: [f32; SIZE],
    perm_x: [usize; SIZE],
    perm_y: [usize; SIZE],
    perm_z: [usize; SIZE],
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ranfloat: Perlin::generate(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f32 {
        let u = point[0] - point[0].floor();
        let v = point[1] - point[1].floor();
        let w = point[2] - point[2].floor();

        let i = point[0].floor() as usize;
        let j = point[1].floor() as usize;
        let k = point[2].floor() as usize;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[(i+di) & 255] ^
                                                  self.perm_y[(j+dj) & 255] ^
                                                  self.perm_z[(k+dk) & 255]];
                }
            }
        }
        Perlin::trilinear_interpolation(c, u, v, w)
    }

    fn generate() -> [f32; SIZE] {
        let mut rng = rand::thread_rng();
        let mut perlin = [0.0; SIZE];
        for i in 0..SIZE {
            perlin[i] = rng.gen();
        }
        perlin
    }

    fn generate_perm() -> [usize; SIZE] {
        let mut perm = [0; SIZE];
        for i in 0..SIZE {
            perm[i] = i as usize;
        }
        Perlin::permute(&mut perm);
        perm
    }

    fn permute(perm: &mut [usize; SIZE]) {
        let mut rng = rand::thread_rng();
        for i in (0..perm.len()).rev() {
            let target = (rng.gen::<f32>() * (i as f32 + 1.0)) as usize;
            perm.swap(i, target);
        }
    }

    fn trilinear_interpolation(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let mut acc = 0.0;
        for i in 0..2 {
            let iu = i as f32 * u;
            for j in 0..2 {
                let jv = j as f32 * v;
                for k in 0..2 {
                    let kw = k as f32 * w;
                    acc += (iu + (1 - i) as f32 * (1.0 - u)) *
                           (jv + (1 - j) as f32 * (1.0 - v)) *
                           (kw + (1 - k) as f32 * (1.0 - w)) *
                           c[i][j][k];
                }
            }
        }
        acc
    }
}