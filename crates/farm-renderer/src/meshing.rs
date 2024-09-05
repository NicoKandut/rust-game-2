use std::ops::AddAssign;
use farm_core::ground::Ground;
use linalg::{Vector3, Vector4};

pub struct Mesh {
    pub vertices: Vec<Vector4>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn with_capacity(v: usize, i: usize) -> Self {
        Self {
            vertices: Vec::<Vector4>::with_capacity(v),
            indices: Vec::<u32>::with_capacity(i),
        }
    }

    pub fn new(vertices: Vec<Vector4>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
        }
    }
}

impl AddAssign<Mesh> for Mesh {
    fn add_assign(&mut self, rhs: Mesh) {
        let offset = self.vertices.len() as u32;
        self.vertices.extend(rhs.vertices);
        self.indices.extend(rhs.indices.iter().map(|i| i + offset));
    }
}

pub fn mesh_ground(ground: &Ground) -> Mesh {
    let mut mesh = Mesh::with_capacity(ground.chunks.len() * 16 * 16 * 4, ground.chunks.len() * 16 * 16 * 6);

    for chunk in ground.chunks.iter() {
        for i in 0..16 {
            for j in 0..16 {
                let tile = chunk.tiles[i][j];
                let quad = mesh_quad(i as f32, j as f32, chunk.z as f32, tile.material.get_palette_x());
                mesh += quad;
            }
        }
    }

    mesh
}

pub fn mesh_quad(x: f32, y: f32, z: f32, w: f32) -> Mesh {
    let vertices = vec![
        Vector4::new(x, y, z, w),
        Vector4::new(x + 1., y, z, w),
        Vector4::new(x, y + 1., z, w),
        Vector4::new(x + 1., y + 1., z, w),
    ];

    let indices = vec![0, 2, 1, 1, 2, 3];

    Mesh::new(vertices, indices)
}