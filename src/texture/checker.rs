use crate::vec3::Vec3;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Checker<T: Texture + Clone, U: Texture + Clone> {
    even: T,
    odd: U,
}

impl<T: Texture + Clone, U: Texture + Clone> Checker<T, U> {
    pub fn new(even: T, odd: U) -> Self {
        Self { even, odd }
    }
}

impl<T: Texture + Clone, U: Texture + Clone> Texture for Checker<T, U> {
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        let sines = (point[0] * 10.0).sin() * (point[1] * 10.0).sin() * (point[2] * 10.0).sin();
        if sines.is_sign_negative() {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}