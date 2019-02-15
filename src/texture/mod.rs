use crate::vec3::Vec3;

mod constant;
pub use constant::Constant;

mod checker;
pub use checker::Checker;

mod noise;
pub use noise::Noise;

mod perlin;

pub trait Texture {
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Vec3;
}