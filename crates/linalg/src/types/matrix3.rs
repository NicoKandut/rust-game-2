#[derive(Debug, Clone, Copy)]
pub struct Matrix3(pub [[f64; 3]; 3]);

impl Matrix3 {
    pub fn from_columns(c1: [f64; 3], c2: [f64; 3], c3: [f64; 3]) -> Self {
        Self([c1, c2, c3])
    }
}
