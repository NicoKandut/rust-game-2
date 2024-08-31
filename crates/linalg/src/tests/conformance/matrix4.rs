use glm::{exp, GenSquareMat};
use crate::tests::conformance::assert_memory_order;

#[test]
fn ones_conformant() {
    let actual = crate::Matrix4::ones();
    let expected = glm::Matrix4::<f32>::new(
        glm::vec4(1.0, 1.0, 1.0, 1.0),
        glm::vec4(1.0, 1.0, 1.0, 1.0),
        glm::vec4(1.0, 1.0, 1.0, 1.0),
        glm::vec4(1.0, 1.0, 1.0, 1.0),
    );

    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(actual.0[i][j], expected[i][j]);
        }
    }

    assert_memory_order(&actual, &expected);
}

#[test]
fn columns_conformant() {
    let actual = crate::Matrix4::from_columns(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let expected = glm::Matrix4::<f32>::new(
        glm::vec4(1.0, 2.0, 3.0, 4.0),
        glm::vec4(5.0, 6.0, 7.0, 8.0),
        glm::vec4(9.0, 10.0, 11.0, 12.0),
        glm::vec4(13.0, 14.0, 15.0, 16.0),
    );

    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(actual.0[i][j], expected[i][j]);
        }
    }

    assert_memory_order(&actual, &expected);
}

#[test]
fn rows_conformant() {
    let actual = crate::Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let expected = glm::mat4(
        1.0, 5.0, 9.0, 13.0,
        2.0, 6.0, 10.0, 14.0,
        3.0, 7.0, 11.0, 15.0,
        4.0, 8.0, 12.0, 16.0,
    );

    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(actual.0[i][j], expected[i][j]);
        }
    }

    assert_memory_order(&actual, &expected);
}

#[test]
fn look_at_conformant() {
    let actual_up = crate::Vector3::new(0.0, 0.0, 1.0);
    let actual_position = crate::Vector3::new(1.0, 2.0, 3.0);
    let actual_target = crate::Vector3::new(4.0, 5.0, 6.0);
    let actual = crate::Matrix4::look_at(actual_position, actual_target, actual_up);

    let expected_up = glm::vec3(0.0, 0.0, 1.0);
    let expected_position = glm::vec3(1.0, 2.0, 3.0);
    let expected_target = glm::vec3(4.0, 5.0, 6.0);
    let expected = glm::ext::look_at_rh(expected_position, expected_target, expected_up);

    // for i in 0..4 {
    //     for j in 0..4 {
    //         assert_eq!(actual.0[i][j], expected[i][j]);
    //     }
    // }

    assert_memory_order(&actual, &expected);
}

#[test]
fn perspective_conformant() {
    let actual = crate::Matrix4::perspective(90.0, 16.0 / 9.0, 0.1, 100.0);
    let mut expected = glm::ext::perspective(90.0, 16.0 / 9.0, 0.1, 100.0);
    expected[1][1] *= -1.0;
    //
    // for i in 0..4 {
    //     for j in 0..4 {
    //         assert_eq!(actual.0[i][j], expected[i][j]);
    //     }
    // }

    assert_memory_order(&actual, &expected);
}

#[test]
fn it_works() {
    let point = crate::Vector4::new(0.0,0.0,0.0, 1.0);
    let glm_point = glm::vec4(0.0, 0.0, 0.0, 1.0);

    let look_at = crate::Matrix4::look_at(
        crate::Vector3::new(-1.0, -1.0, 1.0),
        crate::Vector3::new(0.0, 0.0, 1.0),
        crate::Vector3::new(0.0, 0.0, 1.0),
    );
    let look_at_2 = glm::ext::look_at_rh(
        glm::vec3(-1.0, -1.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
    );

    assert_memory_order(&look_at, &look_at_2);

    let transformed = look_at * point;
    let other = look_at_2 * glm_point;

    println!("{:?} vs {:?}", transformed, other);
}