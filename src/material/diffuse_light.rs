use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::texture::Texture;
use super::Material;

#[derive(Clone)]
pub struct DiffuseLight<T: Texture + Clone> {
    emit: T,
}

impl<T: Texture + Clone> DiffuseLight<T> {
    pub fn new(emit: T) -> Self {
        Self { emit }
    }
}

impl<T: Texture + Clone> Material for DiffuseLight<T> {
    fn scatter(&self, _ray_in: &Ray, _hit_rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}
