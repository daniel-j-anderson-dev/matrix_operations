use super::*;

#[test]
fn mat_mul() {
    let lhs = Matrix::try_from([
        [1, 2, 3, 4], //
        [5, 6, 7, 8],
        [9, 10, 11, 12],
    ])
    .unwrap();

    let rhs = Matrix::try_from([
        [12, 11, 10], //
        [9, 8, 7],
        [6, 5, 4],
        [3, 2, 1],
    ])
    .unwrap();

    let expected_product = Matrix::try_from([
        [60, 50, 40], //
        [180, 154, 128],
        [300, 258, 216],
    ])
    .unwrap();

    let product = lhs.matrix_multiply(&rhs).unwrap();

    assert_eq!(product, expected_product);
}

#[test]
fn scalar_mul() {
    let matrix = Matrix::try_from([
        [5, 5, 5, 5], //
        [5, 5, 5, 5],
        [5, 5, 5, 5],
    ])
    .unwrap();

    let scalar = 2;

    let expected_product = Matrix::try_from([
        [10, 10, 10, 10], //
        [10, 10, 10, 10],
        [10, 10, 10, 10],
    ])
    .unwrap();

    let product = matrix.scalar_multiply(scalar);

    assert_eq!(product, expected_product);
}

#[test]
fn matrix_add() {
    let lhs = Matrix::try_from([
        [1, 2, 3], //
        [1, 2, 3],
        [4, 5, 6],
        [4, 5, 6],
        [4, 5, 6],
    ])
    .unwrap();

    let rhs = Matrix::try_from([
        [-1, -2, -3], //
        [-1, -2, -3],
        [-4, -5, -6],
        [-4, -5, -6],
        [-4, -5, -6],
    ])
    .unwrap();

    let expected_sum = Matrix::try_from([
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ])
    .unwrap();

    let sum = lhs.add(&rhs).unwrap();

    assert_eq!(sum, expected_sum);
}

#[test]
fn minor_ok() {
    let matrix = Matrix::try_from([
        [00, 01, 02, 03], //
        [10, 11, 12, 13],
        [20, 21, 22, 23],
        [30, 31, 32, 33],
    ])
    .unwrap();

    let expected_minor = Matrix::try_from([
        [11, 12, 13], //
        [21, 22, 23],
        [31, 32, 33],
    ])
    .unwrap();

    let minor = matrix.minor((0, 0)).unwrap();

    assert_eq!(minor, expected_minor);
}

#[test]
fn minor_err() {
    let matrix = Matrix::try_from([[1, 2, 3, 4, 5, 6], [1, 2, 3, 4, 5, 6]]).unwrap();

    MatrixError::minor(&matrix, (0, 0)).err().unwrap();
    MatrixError::minor(&matrix, (2, 3)).err().unwrap();
    MatrixError::minor(&matrix, (1, 6)).err().unwrap();
}

#[test]
fn determinant_2x2() {
    let matrix = Matrix::try_from([
        [3, 5], //
        [3, 6],
    ])
    .unwrap();

    let expected_determinant = 3;

    let determinant = matrix.determinant().unwrap();

    assert_eq!(determinant, expected_determinant);
}

#[test]
fn determinant_1x1() {
    let matrix = Matrix::try_from([
        [3], //
    ])
    .unwrap();

    let expected_determinant = 3;

    let determinant = matrix.determinant().unwrap();

    assert_eq!(determinant, expected_determinant);
}

#[test]
fn determinant() {
    let matrix = Matrix::try_from([
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ])
    .unwrap();
    // let matrix2 = Matrix::try_from([
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    //     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    //     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    //     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    //     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    //     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    // ])
    // .unwrap();

    let expected_determinant = 0;

    let determinant = matrix.determinant().unwrap();
    // let determinant2 = matrix2.determinant().unwrap();

    assert_eq!(determinant, expected_determinant);
    // assert_eq!(determinant2, expected_determinant);
}
