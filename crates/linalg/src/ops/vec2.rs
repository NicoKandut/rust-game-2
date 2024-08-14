use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use crate::ops::{Dot, Norm};
use crate::Vector2;

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
    fn dot(self, rhs: Self) -> f64 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

impl Norm for Vector2 {
    fn norm2(&self) -> f64 {
        self.0[0].powi(2) + self.0[1].powi(2)
    }
}

impl Mul<f64> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2([
            self.0[0] * rhs,
            self.0[1] * rhs,
        ])
    }
}