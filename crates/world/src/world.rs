use crate::tree::Tree;

pub struct World {
    tiles: [[u32; 16]; 16],
    trees: Vec<Tree>,
}

impl World {
    pub fn new() -> Self {
        let tiles = [[0u32; 16]; 16];
        let trees = Vec::new();

        Self { tiles, trees }
    }
}