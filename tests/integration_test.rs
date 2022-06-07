use math::linalg::matrix::Matrix;

#[test]
fn test() {
    let m = Matrix::from( [[1, 2], [3, 4], [5, 6]] );
    let m2 = Matrix::from( [[1, 1], [1, 1], [1, 2]] );
    let result = Matrix::from( [[3, 7, 11], [5, 9, 14]] );
    assert_eq!(result, (m * 2 + m2).transpose());
}