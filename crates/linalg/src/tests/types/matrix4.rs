use crate::Matrix4;

#[test]
fn ones_only_includes_ones() {
    let m = Matrix4::ones();
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(m.0[i][j], 1.0);
        }
    }
}

#[test]
fn it_creates_a_matrix4_from_rows() {
    let m = Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );

    assert_eq!(m.0[0][0], 1.0);
    assert_eq!(m.0[1][0], 2.0);
    assert_eq!(m.0[2][0], 3.0);
    assert_eq!(m.0[3][0], 4.0);
    assert_eq!(m.0[0][1], 5.0);
    assert_eq!(m.0[1][1], 6.0);
    assert_eq!(m.0[2][1], 7.0);
    assert_eq!(m.0[3][1], 8.0);
    assert_eq!(m.0[0][2], 9.0);
    assert_eq!(m.0[1][2], 10.0);
    assert_eq!(m.0[2][2], 11.0);
    assert_eq!(m.0[3][2], 12.0);
    assert_eq!(m.0[0][3], 13.0);
    assert_eq!(m.0[1][3], 14.0);
    assert_eq!(m.0[2][3], 15.0);
    assert_eq!(m.0[3][3], 16.0);
}

#[test]
fn it_creates_a_matrix4_from_columns() {
    let m = Matrix4::from_columns(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );

    assert_eq!(m.0[0][0], 1.0);
    assert_eq!(m.0[0][1], 2.0);
    assert_eq!(m.0[0][2], 3.0);
    assert_eq!(m.0[0][3], 4.0);
    assert_eq!(m.0[1][0], 5.0);
    assert_eq!(m.0[1][1], 6.0);
    assert_eq!(m.0[1][2], 7.0);
    assert_eq!(m.0[1][3], 8.0);
    assert_eq!(m.0[2][0], 9.0);
    assert_eq!(m.0[2][1], 10.0);
    assert_eq!(m.0[2][2], 11.0);
    assert_eq!(m.0[2][3], 12.0);
    assert_eq!(m.0[3][0], 13.0);
    assert_eq!(m.0[3][1], 14.0);
    assert_eq!(m.0[3][2], 15.0);
    assert_eq!(m.0[3][3], 16.0);
}