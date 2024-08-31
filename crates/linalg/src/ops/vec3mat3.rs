use std::ops::Mul;

use crate::types::Matrix3;
use crate::types::Vector3;

impl Mul<&Vector3> for &Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        let x = rhs.x();
        let y = rhs.y();
        let z = rhs.z();

        Vector3::new(
            self.0[0][0] * x + self.0[0][1] * y + self.0[0][2] * z,
            self.0[1][0] * x + self.0[1][1] * y + self.0[1][2] * z,
            self.0[2][0] * x + self.0[2][1] * y + self.0[2][2] * z
        )
    }
}