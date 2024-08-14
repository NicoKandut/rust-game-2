#[derive(Debug, Clone, Copy)]
pub struct Matrix2(pub [[f64; 2]; 2]);

impl Matrix2 {
    pub fn from_columns(c1: [f64; 2], c2: [f64; 2]) -> Self {
        Self([c1, c2])
    }
}