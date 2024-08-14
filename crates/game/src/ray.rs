use linalg::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    direction_inv: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        let direction_inv = Vector3::new(
            1.0 / direction.x(),
            1.0 / direction.y(),
            1.0 / direction.z(),
        );
        Self { origin, direction, direction_inv }
    }

    pub fn point_at(&self, t: f64) -> Vector3 {
        &self.origin + &(&self.direction * t)
    }
}