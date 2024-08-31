use linalg::Vector3;

pub struct AABB {
    min: Vector3,
    max: Vector3,
}

impl AABB {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, pos: Vector3) -> bool {
        pos.x() >= self.min.x() && pos.x() < self.max.x() &&
            pos.y() >= self.min.y() && pos.y() < self.max.y() &&
            pos.z() >= self.min.z() && pos.z() < self.max.z()
    }
}