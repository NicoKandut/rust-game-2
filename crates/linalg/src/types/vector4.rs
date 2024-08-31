#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector4(pub(crate) [f32; 4]);

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self([x, y, z, w])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn w(&self) -> f32 {
        self.0[3]
    }
}

impl From<Vector4> for [f32; 4] {
    fn from(v: Vector4) -> Self {
        v.0
    }
}

