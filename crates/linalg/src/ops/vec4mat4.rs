use std::ops::Mul;

use crate::types::Matrix4;
use crate::types::Vector4;

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Vector4 {
        let c1 = self.0[0];
        let c2 = self.0[1];
        let c3 = self.0[2];
        let c4 = self.0[3];

        let x = rhs.x();
        let y = rhs.y();
        let z = rhs.z();
        let w = rhs.w();

        Vector4::new(
            c1[0] * x + c2[0] * y + c3[0] * z + c4[0] * w,
            c1[1] * x + c2[1] * y + c3[1] * z + c4[1] * w,
            c1[2] * x + c2[2] * y + c3[2] * z + c4[2] * w,
            c1[3] * x + c2[3] * y + c3[3] * z + c4[3] * w,
        )
    }
}