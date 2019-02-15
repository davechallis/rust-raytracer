use crate::vec3::Vec3;
use crate::texture::Texture;
use crate::texture::perlin::Perlin;

#[derive(Clone)]
pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn new() -> Self {
        Self { perlin: Perlin::new() }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, point: &Vec3) -> Vec3 {
        Vec3::ones() * self.perlin.noise(point)
    }
}