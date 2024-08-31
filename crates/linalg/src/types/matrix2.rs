#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix2(pub [[f32; 2]; 2]);

impl Matrix2 {
    pub fn from_columns<T: Into<[f32; 2]>>(c1: T, c2: T) -> Self {
        Self([c1.into(), c2.into()])
    }

    pub fn c1(&self) -> [f32; 2] {
        self.0[0]
    }

    pub fn c2(&self) -> [f32; 2] {
        self.0[0]
    }
}