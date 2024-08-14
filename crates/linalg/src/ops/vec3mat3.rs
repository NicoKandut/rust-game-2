use std::ops::Mul;

use crate::types::Matrix3;
use crate::types::Vector3;

impl Mul<&Vector3> for &Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        let a = self.0[0][0];
        let b = self.0[0][1];
        let c = self.0[0][2];
        let d = self.0[1][0];
        let e = self.0[1][1];
        let f = self.0[1][2];
        let g = self.0[2][0];
        let h = self.0[2][1];
        let i = self.0[2][2];

        let x = rhs.0[0];
        let y = rhs.0[0];
        let z = rhs.0[0];

        Vector3::new(a * x + b * y + c * z, d * x + e * y + f * z, g * x + h * y + i * z)
    }
}