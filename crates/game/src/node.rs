use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub(crate) is_leaf: bool,
    pub(crate) material: Material,
    pub(crate) children: Option<[usize; 8]>,
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