use std::collections::HashMap;
use std::mem::size_of;

use linalg::Vector3;

use crate::aabb::AABB;
use crate::node::Node;
use crate::octant::{get_octant_of_vector, get_octant_vector};

#[derive(Debug)]
pub struct Octree {
    root: usize,
    size: usize,
    center: Vector3,
    nodes: HashMap<usize, Node>,
    next_index: usize,
}

impl Default for Octree {
    fn default() -> Self {
        let mut nodes = HashMap::default();
        nodes.insert(0, Node::default());
        Self {
            root: 0,
            size: 2,
            center: Vector3::new(0.0, 0.0, 0.0),
            nodes,
            next_index: 1,
        }
    }
}

impl Octree {
    pub fn get_aabb(&self) -> AABB {
        let half_size = (self.size as f32) / 2.0;
        let half_size3 = Vector3::new(half_size, half_size, half_size);
        let min = self.center - half_size3;
        let max = self.center + half_size3;

        AABB::new(min, max)
    }

    pub fn size_in_bytes(&self) -> usize {
        size_of::<usize>()
            + size_of::<usize>()
            + size_of::<Vector3>()
            + size_of::<Node>() * self.nodes.len()
    }

    pub fn get_voxel(&self, pos: Vector3) -> Option<&Node> {
        let aabb = self.get_aabb();

        if !aabb.contains(pos) {
            return None;
        }


        let mut node = &self.nodes[&self.root];

        while !node.is_leaf {
            let octant = get_octant_of_vector(pos - self.center);
            if let Some(children) = node.children {
                node = &self.nodes[&children[octant]];
            } else {
                return None;
            }
        }

        Some(node)
    }

    pub fn get_voxel_mut(&mut self, pos: Vector3) -> Option<&mut Node> {
        let aabb = self.get_aabb();

        if !aabb.contains(pos) {
            return None;
        }

        let mut index = self.root;
        let mut node = &self.nodes[&index];

        while !node.is_leaf {
            let octant = get_octant_of_vector(pos - self.center);

            if let Some(children) = node.children {
                index = children[octant];
                node = &self.nodes[&index];
            } else {
                return None;
            }
        }

        self.nodes.get_mut(&index)
    }
    pub fn shrink(&mut self, octant: usize) {
        let mut children = self.nodes[&self.root].children;

        if let Some(children) = children {
            self.center = self.center + get_octant_vector(octant) * ((self.size as f32) / 4.0);
            self.size /= 2;

            for index in children {
                if index == octant {
                    self.root = children[octant];
                } else {
                    self.nodes.remove(&index);
                }
            }
        }
    }

    pub fn grow(&mut self, octant: usize) {
        let new_root_id = self.next_index;
        self.next_index += 1;
        let mut new_root = Node::default();
        new_root.is_leaf = false;

        let mut children = [0; 8];
        for index in 0..8 {
            if index == octant {
                children[index] = self.root;
            } else {
                children[index] = self.next_index;
                self.next_index += 1;
                self.nodes.insert(new_root_id, Node::default());
            }
        }
        new_root.children = Some(children);

        self.root = new_root_id;
        self.nodes.insert(new_root_id, new_root);
        self.center = self.center - get_octant_vector(octant) * ((self.size as f32) / 2.0);
        self.size *= 2;
    }
}
