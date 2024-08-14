use linalg::Vector3;

pub struct Camera {
    position: Vector3,
    direction: Vector3,
    up: Vector3,
    right: Vector3,
}