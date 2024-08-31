#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix3(pub [[f32; 3]; 3]);

impl Matrix3 {
    pub fn from_columns<T: Into<[f32; 3]>>(c1: T, c2: T, c3: T) -> Self {
        Self([c1.into(), c2.into(), c3.into()])
    }
}
