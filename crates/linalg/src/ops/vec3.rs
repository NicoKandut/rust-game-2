use crate::ops::{Dot, Norm};
use crate::{Cross, Normalize, Vector3};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
    }
}

impl Dot for Vector3 {
    fn dot(self, rhs: Self) -> f32 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1] + self.0[2] * rhs.0[2]
    }
}

impl Norm for Vector3 {
    fn norm2(&self) -> f32 {
        self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
        ])
    }
}

impl Normalize for Vector3 {
    type Output = Vector3;

    fn normalize(self) -> Self::Output {
        let norm = 1.0 / self.norm();
        Vector3::new(
            self.0[0] * norm,
            self.0[1] * norm,
            self.0[2] * norm,
        )
    }
}

impl Cross for Vector3 {
    type Output = Vector3;
    fn cross(self, rhs: Self) -> Self::Output {
        let x = self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1];
        let y = self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2];
        let z = self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0];
        Vector3::new(x, y, z)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3([-self.0[0], -self.0[1], -self.0[2]])
    }
}