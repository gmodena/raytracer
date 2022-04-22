use std::ops::{Neg, Add, Sub, Div, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    fn length_squared(self) -> f32 {
        f32::powi(self.0, 2) + f32::powi(self.1, 2) + f32::powi(self.2, 2)
    }
    fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }
    fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }
    fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3(u.1 * v.2 - u.2 * v.1, u.0 * v.0 - u.0 * v.2, u.0 * v.1 - u.1 * v.0)
    }

    fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}


impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self::Output {
        self * (1.0/t)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self::Output {
        Vec3(self.0, self.1 * t, self.2 * t)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

pub type Color = Vec3;
pub type Point3D = Vec3;
