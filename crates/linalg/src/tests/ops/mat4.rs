use crate::Matrix4;

#[test]
fn it_adds_two_matrix4() {
    let m1 = Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let m2 = Matrix4::from_rows(
        [16.0, 15.0, 14.0, 13.0],
        [12.0, 11.0, 10.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    );
    let expected = Matrix4::from_rows(
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
    );
    let actual = m1 + m2;
    assert_eq!(actual, expected);
}

#[test]
fn it_add_assigns_two_matrix4() {
    let mut m1 = Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let m2 = Matrix4::from_rows(
        [16.0, 15.0, 14.0, 13.0],
        [12.0, 11.0, 10.0, 9.0],
        [8.0, 7.0, 6.0, 5.0],
        [4.0, 3.0, 2.0, 1.0],
    );
    let expected = Matrix4::from_rows(
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
        [17.0, 17.0, 17.0, 17.0],
    );
    m1 += m2;
    assert_eq!(m1, expected);
}

#[test]
fn it_subtracts_two_matrix4() {
    let m1 = Matrix4::from_rows(
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [10.0, 11.0, 12.0, 13.0],
        [14.0, 15.0, 16.0, 17.0],
    );
    let m2 = Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let expected = Matrix4::ones();
    let actual = m1 - m2;
    assert_eq!(actual, expected);
}

#[test]
fn it_subtracts_assigns_two_matrix4() {
    let mut m1 = Matrix4::from_rows(
        [2.0, 3.0, 4.0, 5.0],
        [6.0, 7.0, 8.0, 9.0],
        [10.0, 11.0, 12.0, 13.0],
        [14.0, 15.0, 16.0, 17.0],
    );
    let m2 = Matrix4::from_rows(
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
        [13.0, 14.0, 15.0, 16.0],
    );
    let expected = Matrix4::ones();
    m1 -= m2;
    assert_eq!(m1, expected);
}