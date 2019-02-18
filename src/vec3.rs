use std::ops::{Add, AddAssign, Neg, Div, DivAssign, Sub, SubAssign, Mul, MulAssign, Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn zeros() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn to_unit_vector(&self) -> Vec3 {
        let length = self.length();
        self * (1.0 / length)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(self.e[1] * other.e[2] - self.e[2] * other.e[1],
                  -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
                  self.e[0] * other.e[1] - self.e[1] * other.e[0])
    }
}


impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}




impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Vec3 {
            e: [self.e[0] + scalar,
                self.e[1] + scalar,
                self.e[2] + scalar]
        }
    }
}

impl Add<f32> for &Vec3 {
    type Output = Vec3;

    fn add(self, scalar: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] + scalar,
                self.e[1] + scalar,
                self.e[2] + scalar]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}





impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}


impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, scalar: f32) -> Self {
        Vec3 {
            e: [self.e[0] - scalar,
                self.e[1] - scalar,
                self.e[2] - scalar]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            e: [self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]]
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]]
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vec3 {
            e: [self.e[0] * scalar,
                self.e[1] * scalar,
                self.e[2] * scalar]
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * scalar,
                self.e[1] * scalar,
                self.e[2] * scalar]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        vector * self
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vector: &Vec3) -> Vec3 {
        vector * self
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            e: [self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2]]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Vec3 {
            e: [self.e[0] / scalar,
                self.e[1] / scalar,
                self.e[2] / scalar]
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / scalar,
                self.e[1] / scalar,
                self.e[2] / scalar]
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.e[0] /= scalar;
        self.e[1] /= scalar;
        self.e[2] /= scalar;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        match idx {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => panic!("Index out of bounds: {}", idx),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f32 {
        match idx {
            0 => &mut self.e[0],
            1 => &mut self.e[1],
            2 => &mut self.e[2],
            _ => panic!("Index out of bounds: {}", idx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_eq() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec3_basic_ops() {
        let v1 = Vec3::new(1.0, 2.0, 4.0);
        let v2 = Vec3::new(4.0, 2.0, 1.0);
        assert_eq!(v1 + v2, Vec3::new(5.0, 4.0, 5.0));
        assert_eq!(v1 - v2, Vec3::new(-3.0, 0.0, 3.0));
        assert_eq!(v1 * v2, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(v1 / v2, Vec3::new(0.25, 1.0, 4.0));
    }

    #[test]
    fn test_vec3_assign_ops() {
        let mut v1 = Vec3::new(0.5, -0.5, 1.0);
        let v2 = Vec3::new(1.0, -2.0, 3.0);

        v1 += v2;
        assert_eq!(v1, Vec3::new(1.5, -2.5, 4.0));

        v1 -= v2;
        assert_eq!(v1, Vec3::new(0.5, -0.5, 1.0));

        v1 *= v2;
        assert_eq!(v1, Vec3::new(0.5, 1.0, 3.0));

        v1 /= v2;
        assert_eq!(v1, Vec3::new(0.5, -0.5, 1.0));
    }

    #[test]
    fn test_vec3_scalar_ops() {
        let v1 = Vec3::new(1.0, -2.0, 4.0);
        assert_eq!(v1 + 1.5, Vec3::new(2.5, -0.5, 5.5));
        assert_eq!(v1 - 0.5, Vec3::new(0.5, -2.5, 3.5));
        assert_eq!(v1 * -1.5, Vec3::new(-1.5, 3.0, -6.0));
        assert_eq!(v1 / 2.0, Vec3::new(0.5, -1.0, 2.0));
    }

    #[test]
    fn test_vec3_length() {
        let v1 = Vec3::new(1.0, -1.0, -1.0);
        assert_eq!(v1.length(), 3.0f32.sqrt());

        let v1 = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(v1.length(), 0.0);

        let v1 = Vec3::new(-1.0, 0.0, 4.0);
        assert_eq!(v1.length(), 17.0f32.sqrt());
    }
}
