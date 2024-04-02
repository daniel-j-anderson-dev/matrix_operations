use super::*;

#[test]
fn matrix_multiplication() {
    let lhs = Matrix::from([
        [1, 2, 3, 4], //
        [5, 6, 7, 8],
        [9, 10, 11, 12],
    ]);

    let rhs = Matrix::from([
        [12, 11, 10], //
        [9, 8, 7],
        [6, 5, 4],
        [3, 2, 1],
    ]);

    let expected_product = Matrix::from([
        [60, 50, 40], //
        [180, 154, 128],
        [300, 258, 216],
    ]);

    let product = lhs.matrix_multiply(&rhs).unwrap();

    assert!(product == expected_product);
}

#[test]
fn scalar_mul() {
    let matrix = Matrix::from([
        [5, 5, 5, 5], //
        [5, 5, 5, 5],
        [5, 5, 5, 5],
    ]);

    let scalar = 2;

    let expected_product = Matrix::from([
        [10, 10, 10, 10], //
        [10, 10, 10, 10],
        [10, 10, 10, 10],
    ]);

    let product = matrix.scalar_multiply(scalar);

    assert!(product == expected_product);
}

#[test]
fn matrix_add() {
    let lhs = Matrix::from([
        [1, 2, 3], //
        [1, 2, 3],
        [4, 5, 6],
        [4, 5, 6],
        [4, 5, 6],
    ]);

    let rhs = Matrix::from([
        [-1, -2, -3], //
        [-1, -2, -3],
        [-4, -5, -6],
        [-4, -5, -6],
        [-4, -5, -6],
    ]);

    let expected_sum = Matrix::from([
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ]);

    let sum = lhs.add(&rhs).unwrap();

    assert!(sum == expected_sum);
}

#[test]
fn minor_ok() {
    let matrix = Matrix::from([
        [0.5, 1.2, 1.22, -2.1],
        [-1.1, -2.5, 0.6, 0.0],
        [0.0, 1.1, -2.2, -0.1],
        [-3.0, 1.2, -0.5, 1.0],
    ]);

    let expected_minor = Matrix::from([
        [-2.5, 0.6, 0.0], //
        [1.1, -2.2, -0.1],
        [1.2, -0.5, 1.0],
    ]);

    let minor = matrix.minor(0, 0).unwrap();

    assert!(minor == expected_minor);
}

#[test]
fn minor_err() {
    let matrix = Matrix::from([[1, 2, 3, 4, 5, 6], [1, 2, 3, 4, 5, 6]]);

    MatrixError::minor(&matrix, 0, 0).err().unwrap();
    MatrixError::minor(&matrix, 2, 0).err().unwrap();
    MatrixError::minor(&matrix, 0, 6).err().unwrap();
}

#[test]
fn determinant() {
    let matrix = Matrix::from([
        [0.5, 1.2, 1.22, -2.1],
        [-1.1, -2.5, 0.6, 0.0],
        [0.0, 1.1, -2.2, -0.1],
        [-3.0, 1.2, -0.5, 1.0],
    ]);

    let expected_determinant = -38.61164;

    let determinant = matrix.determinant().unwrap();

    println!("{}\n{}", expected_determinant, determinant);

    assert!(determinant == expected_determinant);
}
