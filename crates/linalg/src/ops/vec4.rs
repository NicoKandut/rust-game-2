use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use crate::ops::{Dot, Norm};
use crate::{Normalize, Vector4};

impl Add for &Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Self) -> Self::Output {
        Vector4([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
        self.0[3] += rhs.0[3];
    }
}

impl Sub for &Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector4([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
        self.0[3] -= rhs.0[3];
    }
}

impl Dot for Vector4 {
    fn dot(self, rhs: Self) -> f32 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2] + self.0[3] * rhs.0[3]
    }
}

impl Norm for &Vector4 {
    fn norm2(&self) -> f32 {
        self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)+ self.0[3].powi(2)
    }
}

impl Mul<f32> for &Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}

impl Normalize for &Vector4 {
    type Output = Vector4;
    fn normalize(self) -> Self::Output {
        let norm = self.norm();
        Vector4([
            self.0[0] / norm,
            self.0[1] / norm,
            self.0[2] / norm,
            self.0[3] / norm,
        ])
    }
}

impl Neg for &Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}