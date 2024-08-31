use std::slice::from_raw_parts;

pub fn assert_memory_order(actual: &crate::Vector3, expected: &glm::Vector3<f32>) {
    unsafe {
        let actual_bytes = from_raw_parts(&actual.0 as *const [f32; 3] as *const f32, 3);
        let expected_bytes = from_raw_parts(expected as *const glm::Vector3<f32> as *const f32, 3);
        assert_eq!(actual_bytes, expected_bytes);
    }
}

#[test]
fn new_conformance() {
    let actual = crate::Vector3::new(1., 2., 3.);
    let expected = glm::vec3(1., 2., 3.);
    assert_memory_order(&actual, &expected);
}