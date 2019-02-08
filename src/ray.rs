use crate::vec3::Vec3;

#[derive(Clone, PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        let tmp = t * &self.b;
        &self.a + &tmp
    }
}
