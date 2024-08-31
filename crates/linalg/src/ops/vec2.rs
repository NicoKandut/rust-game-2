use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::ops::{Dot, Norm};
use crate::{Normalize, Vector2};

impl Add for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2([
            self.0[0] + rhs.0[1],
            self.0[1] + rhs.0[1],
        ])
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
    }
}

impl Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
        ])
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
    }
}

impl Dot for Vector2 {
    fn dot(self, rhs: Self) -> f32 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

impl Norm for Vector2 {
    fn norm2(&self) -> f32 {
        self.0[0].powi(2) + self.0[1].powi(2)
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2([
            self.0[0] * rhs,
            self.0[1] * rhs,
        ])
    }
}

impl Normalize for Vector2 {
    type Output = Vector2;
    fn normalize(self) -> Self::Output {
        let norm = self.norm();
        Vector2([
            self.0[0] / norm,
            self.0[1] / norm,
        ])
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2([-self.0[0], -self.0[1]])
    }
}