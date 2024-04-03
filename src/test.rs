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

    assert!(product == expected_product);
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

    assert!(product == expected_product);
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

    assert!(sum == expected_sum);
}

#[test]
fn minor_ok() {
    let matrix = Matrix::try_from([
        [0.5, 1.2, 1.22, -2.1],
        [-1.1, -2.5, 0.6, 0.0],
        [0.0, 1.1, -2.2, -0.1],
        [-3.0, 1.2, -0.5, 1.0],
    ])
    .unwrap();

    let expected_minor = Matrix::try_from([
        [-2.5, 0.6, 0.0], //
        [1.1, -2.2, -0.1],
        [1.2, -0.5, 1.0],
    ])
    .unwrap();

    let minor = matrix.minor(0, 0).unwrap();

    assert!(minor == expected_minor);
}

#[test]
fn minor_err() {
    let matrix = Matrix::try_from([[1, 2, 3, 4, 5, 6], [1, 2, 3, 4, 5, 6]]).unwrap();

    MatrixError::minor(&matrix, 0, 0).err().unwrap();
    MatrixError::minor(&matrix, 2, 0).err().unwrap();
    MatrixError::minor(&matrix, 0, 6).err().unwrap();
}

#[test]
fn determinant_2x2() {
    let matrix = Matrix::try_from([[0; 0]; 0]).unwrap();
}

#[test]
fn determinant() {
    let matrix = Matrix::try_from([
        [0.5, 1.2, 1.22, -2.1],
        [-1.1, -2.5, 0.6, 0.0],
        [0.0, 1.1, -2.2, -0.1],
        [-3.0, 1.2, -0.5, 1.0],
    ])
    .unwrap();

    let expected_determinant = -38.61164;

    let determinant = matrix.determinant().unwrap();

    println!("{}\n{}", expected_determinant, determinant);

    assert!(determinant == expected_determinant);
}
