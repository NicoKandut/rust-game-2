#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3(pub(crate) [f32; 3]);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
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

    pub fn mut_x(&mut self) -> &mut f32 {
        &mut self.0[0]
    }

    pub fn mut_y(&mut self) -> &mut f32 {
        &mut self.0[1]
    }

    pub fn mut_z(&mut self) -> &mut f32 {
        &mut self.0[2]
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(v: Vector3) -> Self {
        v.0
    }
}

