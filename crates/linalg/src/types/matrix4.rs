use crate::{Cross, Dot, Normalize, Vector3};

/// Stores a 4x4 matrix.
/// column-major order
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn ones() -> Self {
        Self([
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        ])
    }

    pub fn from_columns<T: Into<[f32; 4]>>(c1: T, c2: T, c3: T, c4: T) -> Self {
        Self([c1.into(), c2.into(), c3.into(), c4.into()])
    }

    pub fn from_rows<T: Into<[f32; 4]>>(c1: T, c2: T, c3: T, c4: T) -> Self {
        let c1 = c1.into();
        let c2 = c2.into();
        let c3 = c3.into();
        let c4 = c4.into();

        Self([
            [c1[0], c2[0], c3[0], c4[0]],
            [c1[1], c2[1], c3[1], c4[1]],
            [c1[2], c2[2], c3[2], c4[2]],
            [c1[3], c2[3], c3[3], c4[3]],
        ])
    }

    pub fn look_at(position: Vector3, target: Vector3, up: Vector3) -> Self {
        let f = (target - position).normalize(); // correct
        let r = f.cross(up.normalize()).normalize(); // correct
        let u = r.cross(f); // shaky but not much room for error

        Self::from_columns(
            [r.x(), u.x(), -f.x(), 0.0],
            [r.y(), u.y(), -f.y(), 0.0],
            [r.z(), u.z(), -f.z(), 0.0],
            [-r.dot(position), -u.dot(position), f.dot(position), 1.0],
        )
    }

    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let q = 1.0 / (fov / 2.0).tan();
        let a = q / aspect;
        let b = (near + far) / (near - far);
        let c = (2.0 * near * far) / (near - far);

        Matrix4::from_columns(
            [a, 0., 0., 0.],
            [0., -q, 0., 0.],
            [0., 0., b, -1.0],
            [0., 0., c, 0.],
        )
    }
}
