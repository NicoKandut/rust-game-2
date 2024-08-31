use linalg::{Identity, Matrix4};

#[repr(C)]
pub(crate) struct UniformBufferObject {
    pub model: Matrix4,
    pub view: Matrix4,
    pub proj: Matrix4,
}

impl Default for UniformBufferObject {
    fn default() -> Self {
        Self {
            model: Matrix4::identity(),
            view: Matrix4::identity(),
            proj: Matrix4::identity(),
        }
    }
}