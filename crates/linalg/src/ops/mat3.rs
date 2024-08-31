use crate::{Identity, Matrix3};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

impl Identity for Matrix3 {
    fn identity() -> Matrix3 {
        Matrix3([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }
}

impl Add for &Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix3([
            [self.0[0][0] + rhs.0[0][0], self.0[0][1] + rhs.0[0][1], self.0[0][2] + rhs.0[0][2]],
            [self.0[1][0] + rhs.0[1][0], self.0[1][1] + rhs.0[1][1], self.0[1][2] + rhs.0[1][2]],
            [self.0[2][0] + rhs.0[2][0], self.0[2][1] + rhs.0[2][1], self.0[2][2] + rhs.0[2][2]],
        ])
    }
}

impl AddAssign for Matrix3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0][0] += rhs.0[0][0];
        self.0[0][1] += rhs.0[0][1];
        self.0[0][2] += rhs.0[0][2];
        self.0[1][0] += rhs.0[1][0];
        self.0[1][1] += rhs.0[1][1];
        self.0[1][2] += rhs.0[1][2];
        self.0[2][0] += rhs.0[2][0];
        self.0[2][1] += rhs.0[2][1];
        self.0[2][2] += rhs.0[2][2];
    }
}

impl Sub for Matrix3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Matrix3([
            [self.0[0][0] - rhs.0[0][0], self.0[0][1] - rhs.0[0][1], self.0[0][2] - rhs.0[0][2]],
            [self.0[1][0] - rhs.0[1][0], self.0[1][1] - rhs.0[1][1], self.0[1][2] - rhs.0[1][2]],
            [self.0[2][0] - rhs.0[2][0], self.0[2][1] - rhs.0[2][1], self.0[2][2] - rhs.0[2][2]],
        ])
    }
}

impl SubAssign for Matrix3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0][0] -= rhs.0[0][0];
        self.0[0][1] -= rhs.0[0][1];
        self.0[0][2] -= rhs.0[0][2];
        self.0[1][0] -= rhs.0[1][0];
        self.0[1][1] -= rhs.0[1][1];
        self.0[1][2] -= rhs.0[1][2];
        self.0[2][0] -= rhs.0[2][0];
        self.0[2][1] -= rhs.0[2][1];
        self.0[2][2] -= rhs.0[2][2];
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = self.0[0][0];
        let b = self.0[0][1];
        let c = self.0[0][2];
        let d = self.0[1][0];
        let e = self.0[1][1];
        let f = self.0[1][2];
        let g = self.0[2][0];
        let h = self.0[2][1];
        let i = self.0[2][2];

        let j = rhs.0[0][0];
        let k = rhs.0[0][1];
        let l = rhs.0[0][2];
        let m = rhs.0[1][0];
        let n = rhs.0[1][1];
        let o = rhs.0[1][2];
        let p = rhs.0[2][0];
        let q = rhs.0[2][1];
        let r = rhs.0[2][2];

        Matrix3([
            [
                a * j + b * m + c * p,
                a * k + b * n + c * q,
                a * l + b * o + c * r,
            ],
            [
                d * j + e * m + f * p,
                d * k + e * n + f * q,
                d * l + e * o + f * r,
            ],
            [
                g * j + h * m + i * p,
                g * k + h * n + i * q,
                g * l + h * o + i * r,
            ],
        ])
    }
}