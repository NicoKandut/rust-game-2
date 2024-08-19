use crate::material::Material;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Node {
    pub is_leaf: bool,
    pub material: Material,
    pub children: Option<[usize; 8]>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            is_leaf: true,
            material: Material(0),
            children: None,
        }
    }
}