#[derive(Debug, Clone, Copy)]
pub struct Vector2(pub(crate) [f64; 2]);

impl Vector2 {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self([x, y])
    }

    pub(crate) fn x(&self) -> f64 {
        self.0[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self.0[1]
    }
}

