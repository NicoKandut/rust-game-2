use std::slice::from_raw_parts;

mod matrix4;
mod vector3;

pub fn assert_memory_order(actual: &crate::Matrix4, expected: &glm::Matrix4<f32>) {
    unsafe {
        let actual_bytes = from_raw_parts(&actual.0 as *const [[f32; 4]; 4] as *const f32, 16);
        let expected_bytes = from_raw_parts(expected as *const glm::Matrix4<f32> as *const f32, 16);
        assert_eq!(actual_bytes, expected_bytes);
    }
}