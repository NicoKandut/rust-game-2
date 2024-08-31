use linalg::Vector3;

pub fn get_octant_vector(octant: usize) -> Vector3 {
    let x = if octant & 1 != 0 { 1.0 } else { -1.0 };
    let y = if octant & 2 != 0 { 1.0 } else { -1.0 };
    let z = if octant & 4 != 0 { 1.0 } else { -1.0 };
    Vector3::new(x, y, z)
}

pub fn get_octant_of_vector(vector: Vector3) -> usize {
    let x = if vector.x() >= 0.0 { 1 } else { 0 };
    let y = if vector.y() >= 0.0 { 1 } else { 0 };
    let z = if vector.z() >= 0.0 { 1 } else { 0 };
    (x << 2) | (y << 1) | z
}
