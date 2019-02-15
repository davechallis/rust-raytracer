use crate::vec3::Vec3;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Constant {
    colour: Vec3,
}

impl Constant {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { colour: Vec3::new(r, g, b) }
    }
}

impl Texture for Constant {
    fn value(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        self.colour.clone()
    }
}