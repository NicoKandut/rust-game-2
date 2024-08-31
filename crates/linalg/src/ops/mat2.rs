use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use crate::{Identity, Matrix2};

impl Identity for Matrix2 {
    fn identity() -> Self {
        let c1 = [1.0, 0.0];
        let c2 = [0.0, 1.0];
        Matrix2([c1, c2])
    }
}

impl Add for Matrix2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let self_c1 = self.c1();
        let self_c2 = self.c2();
        let rhs_c1 = rhs.c1();
        let rhs_c2 = rhs.c2();
        let c1 = [self_c1[0] + rhs_c1[0], self_c1[1] + rhs_c1[1]];
        let c2 = [self_c2[0] + rhs_c2[0], self_c2[1] + rhs_c2[1]];
        Matrix2([c1, c2])
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