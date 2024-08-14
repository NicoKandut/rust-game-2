use std::ops::Mul;
use crate::types::Matrix2;
use crate::types::Vector2;

impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        let a = self.0[0][0];
        let b = self.0[0][1];
        let c = self.0[1][0];
        let d = self.0[1][1];

        let x = rhs.x();
        let y = rhs.y();

        Vector2::new(a * x + b * y, c * x + d * y)
    }
}