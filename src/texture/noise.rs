use crate::vec3::Vec3;
use crate::texture::Texture;
use crate::texture::perlin::Perlin;

#[derive(Clone)]
pub struct Noise {
    /// Scaling applied to input points to change frequency of Perlin noise.
    scale: f32,
    perlin: Perlin,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Self { scale, perlin: Perlin::new() }
    }

    fn turbulence(&self, point: &Vec3, depth: u32) -> f32 {
        let mut acc = 0.0;
        let mut temp_point = point.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            acc += weight * self.perlin.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.0;
        }

        acc.abs()
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, point: &Vec3) -> Vec3 {
        //Vec3::new(0.5, 0.5, 0.5) * (1.0 + self.turbulence(&(self.scale * point), 7))
        //Vec3::ones() * self.turbulence(&(self.scale * point), 7)
        Vec3::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * point[0] + 5.0 * self.turbulence(&(self.scale * point), 7)).sin())
    }
}