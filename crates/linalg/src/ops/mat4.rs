use crate::{Identity, Matrix4};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

impl Identity for Matrix4 {
    fn identity() -> Self {
        Matrix4::from_rows(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
    }
}

impl Add for Matrix4 {
    type Output = Matrix4;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4([
            [self.0[0][0] + rhs.0[0][0], self.0[0][1] + rhs.0[0][1], self.0[0][2] + rhs.0[0][2], self.0[0][3] + rhs.0[0][3]],
            [self.0[1][0] + rhs.0[1][0], self.0[1][1] + rhs.0[1][1], self.0[1][2] + rhs.0[1][2], self.0[1][3] + rhs.0[1][3]],
            [self.0[2][0] + rhs.0[2][0], self.0[2][1] + rhs.0[2][1], self.0[2][2] + rhs.0[2][2], self.0[2][3] + rhs.0[2][3]],
            [self.0[3][0] + rhs.0[3][0], self.0[3][1] + rhs.0[3][1], self.0[3][2] + rhs.0[3][2], self.0[3][3] + rhs.0[3][3]],
        ])
    }
}

impl AddAssign for Matrix4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl Sub for Matrix4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Matrix4([
            [self.0[0][0] - rhs.0[0][0], self.0[0][1] - rhs.0[0][1], self.0[0][2] - rhs.0[0][2], self.0[0][3] - rhs.0[0][3]],
            [self.0[1][0] - rhs.0[1][0], self.0[1][1] - rhs.0[1][1], self.0[1][2] - rhs.0[1][2], self.0[1][3] - rhs.0[1][3]],
            [self.0[2][0] - rhs.0[2][0], self.0[2][1] - rhs.0[2][1], self.0[2][2] - rhs.0[2][2], self.0[2][3] - rhs.0[2][3]],
            [self.0[3][0] - rhs.0[3][0], self.0[3][1] - rhs.0[3][1], self.0[3][2] - rhs.0[3][2], self.0[3][3] - rhs.0[3][3]],
        ])
    }
}

impl SubAssign for Matrix4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0][0] -= rhs.0[0][0];
        self.0[0][1] -= rhs.0[0][1];
        self.0[0][2] -= rhs.0[0][2];
        self.0[0][3] -= rhs.0[0][3];
        self.0[1][0] -= rhs.0[1][0];
        self.0[1][1] -= rhs.0[1][1];
        self.0[1][2] -= rhs.0[1][2];
        self.0[1][3] -= rhs.0[1][3];
        self.0[2][0] -= rhs.0[2][0];
        self.0[2][1] -= rhs.0[2][1];
        self.0[2][2] -= rhs.0[2][2];
        self.0[2][3] -= rhs.0[2][3];
        self.0[3][0] -= rhs.0[3][0];
        self.0[3][1] -= rhs.0[3][1];
        self.0[3][2] -= rhs.0[3][2];
        self.0[3][3] -= rhs.0[3][3];
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = self.0[0][0];
        let b = self.0[0][1];
        let c = self.0[0][2];
        let d = self.0[0][3];
        let e = self.0[1][0];
        let f = self.0[1][1];
        let g = self.0[1][2];
        let h = self.0[1][3];
        let i = self.0[2][0];
        let j = self.0[2][1];
        let k = self.0[2][2];
        let l = self.0[2][3];
        let m = self.0[3][0];
        let n = self.0[3][1];
        let o = self.0[3][2];
        let p = self.0[3][3];

        let q = rhs.0[0][0];
        let r = rhs.0[0][1];
        let s = rhs.0[0][2];
        let t = rhs.0[0][3];
        let u = rhs.0[1][0];
        let v = rhs.0[1][1];
        let w = rhs.0[1][2];
        let x = rhs.0[1][3];
        let y = rhs.0[2][0];
        let z = rhs.0[2][1];
        let aa = rhs.0[2][2];
        let ab = rhs.0[2][3];
        let ac = rhs.0[3][0];
        let ad = rhs.0[3][1];
        let ae = rhs.0[3][2];
        let af = rhs.0[3][3];

        Matrix4([
            [
                a * q + b * u + c * y + d * ac,
                a * r + b * v + c * z + d * ad,
                a * s + b * w + c * aa + d * ae,
                a * t + b * x + c * ab + d * af,
            ],
            [
                e * q + f * u + g * y + h * ac,
                e * r + f * v + g * z + h * ad,
                e * s + f * w + g * aa + h * ae,
                e * t + f * x + g * ab + h * af,
            ],
            [
                i * q + j * u + k * y + l * ac,
                i * r + j * v + k * z + l * ad,
                i * s + j * w + k * aa + l * ae,
                i * t + j * x + k * ab + l * af,
            ],
            [
                m * q + n * u + o * y + p * ac,
                m * r + n * v + o * z + p * ad,
                m * s + n * w + o * aa + p * ae,
                m * t + n * x + o * ab + p * af,
            ],
        ])
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.0[i][j] != other.0[i][j] {
                    return false;
                }
            }
        }
        true
    }
}