#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2(pub(crate) [f32; 2]);

impl Vector2 {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }

    pub(crate) fn x(&self) -> f32 {
        self.0[0]
    }

    pub(crate) fn y(&self) -> f32 {
        self.0[1]
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(v: Vector2) -> Self {
        v.0
    }
}