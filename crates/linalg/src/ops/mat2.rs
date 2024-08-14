use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use crate::Matrix2;

impl Add for Matrix2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Matrix2([
            [self.0[0][0] + rhs.0[0][0], self.0[0][1] + rhs.0[0][1]],
            [self.0[1][0] + rhs.0[1][0], self.0[1][1] + rhs.0[1][1]],
        ])
    }
}

impl AddAssign for Matrix2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0][0] += rhs.0[0][0];
        self.0[0][1] += rhs.0[0][1];
        self.0[1][0] += rhs.0[1][0];
        self.0[1][1] += rhs.0[1][1];
    }
}

impl Sub for Matrix2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Matrix2([
            [self.0[0][0] - rhs.0[0][0], self.0[0][1] - rhs.0[0][1]],
            [self.0[1][0] - rhs.0[1][0], self.0[1][1] - rhs.0[1][1]],
        ])
    }
}

impl SubAssign for Matrix2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0][0] -= rhs.0[0][0];
        self.0[0][1] -= rhs.0[0][1];
        self.0[1][0] -= rhs.0[1][0];
        self.0[1][1] -= rhs.0[1][1];
    }
}

impl Mul for Matrix2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = self.0[0][0];
        let b = self.0[0][1];
        let c = self.0[1][0];
        let d = self.0[1][1];

        let e = rhs.0[0][0];
        let f = rhs.0[0][1];
        let g = rhs.0[1][0];
        let h = rhs.0[1][1];

        Matrix2([
            [a * e + b * g, a * f + b * h],
            [c * e + d * g, c * f + d * h],
        ])
    }
}