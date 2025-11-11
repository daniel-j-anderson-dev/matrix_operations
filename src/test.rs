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
#[should_panic]
fn minor_err() {
    let matrix = Matrix::try_from([
        [1, 2, 3, 4, 5, 6], //
        [1, 2, 3, 4, 5, 6],
    ])
    .unwrap();

    matrix.minor((0, 0)).unwrap();
    matrix.minor((2, 3)).unwrap();
    matrix.minor((1, 6)).unwrap();
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
fn determinant_10x10() {
    let matrix2 = Matrix::try_from([
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ])
    .unwrap();
    let expected_determinant = 0;
    let determinant = matrix2.determinant().unwrap();
    assert_eq!(determinant, expected_determinant);
}

#[test]
fn determinant3x3() {
    let matrix = Matrix::try_from([
        [1, 2, 3], //
        [4, 5, 6],
        [7, 8, 9],
    ])
    .unwrap();

    let expected_determinant = 0;

    let determinant = matrix.determinant().unwrap();

    assert_eq!(determinant, expected_determinant);
}

#[test]
fn inverse3x3_1() {
    let matrix = Matrix::try_from([
        [1.0, 2.0, 3.0], //
        [0.0, 1.0, 4.0],
        [5.0, 6.0, 0.0],
    ])
    .unwrap();

    let inverse = matrix.inverse().unwrap();

    let expected_inverse = Matrix::try_from([
        [-24.0, 18.0, 5.0], //
        [20.0, -15.0, -4.0],
        [-5.0, 4.0, 1.0],
    ])
    .unwrap();

    assert_eq!(inverse, expected_inverse);

    let identity = matrix.matrix_multiply(&inverse).unwrap();
    let expected_identity = Matrix::<f64>::identity(matrix.width_nonzero());
    assert_eq!(identity, expected_identity);
}

#[test]
fn inverse3x3_2() {
    let matrix = Matrix::try_from([
        [2.0, 4.0, -6.0], //
        [7.0, 3.0, 5.0],
        [1.0, -2.0, 4.0],
    ])
    .unwrap();

    let mut inverse = matrix.inverse().unwrap();
    // round each element
    for element in inverse.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    let mut expected_inverse = Matrix::try_from([
        [11.0 / 27.0, -2.0 / 27.0, 19.0 / 27.0], //
        [-23.0 / 54.0, 7.0 / 27.0, -26.0 / 27.0],
        [-17.0 / 54.0, 4.0 / 27.0, -11.0 / 27.0],
    ])
    .unwrap();
    // round each element
    for element in expected_inverse.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    assert_eq!(inverse, expected_inverse);

    let identity = matrix.matrix_multiply(&inverse).unwrap();
    let expected_identity = Matrix::<f64>::identity(matrix.width_nonzero());
    for (&actual, &expected) in identity.elements().zip(expected_identity.elements()) {
        assert!((expected - actual).abs() <= 0.001);
    }
}

#[test]
fn inverse2x2() {
    let matrix = Matrix::try_from([
        [7.0, 38.5], //
        [38.5, 218.75],
    ])
    .unwrap();

    let mut inverse = matrix.inverse().unwrap();
    // round each element
    for element in inverse.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    let mut expected_inverse = Matrix::try_from([
        [4.4643, -0.78571], //
        [-0.78571, 0.14286],
    ])
    .unwrap();
    // round each element
    for element in expected_inverse.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    assert_eq!(inverse, expected_inverse);

    let identity = matrix.matrix_multiply(&inverse).unwrap();
    let expected_identity = Matrix::<f64>::identity(matrix.width_nonzero());
    for (actual, expected) in identity.elements().zip(expected_identity.elements()) {
        assert!((expected - actual).abs() <= 0.01);
    }
}

#[test]
fn transpose() {
    let matrix = Matrix::try_from([
        [1, 4], //
        [2, 5],
        [3, 6],
    ])
    .unwrap();

    let expected_transpose = Matrix::try_from([
        [1, 2, 3], //
        [4, 5, 6],
    ])
    .unwrap();

    let transpose = matrix.transpose();

    assert_eq!(transpose, expected_transpose);
}

#[test]
fn parse_data_set() {
    const DATA: &str = "4.5, 42.0\n5.0, 45.0\n5.5, 51.0\n6.0, 53.0\n6.5, 61.0\n7.0, 62.0";

    DATA.parse::<Data<f64>>().unwrap();
}

#[test]
fn read_data_set() {
    Data::<f64>::from_csv("./tests/dataset.csv").unwrap();
}

#[test]
fn linear_regression() {
    let data = Data::<f64>::from_csv("./tests/dataset.csv").unwrap();
    let mut coefficient_matrix = data.polynomial_regression(1).unwrap();
    // round each element
    for element in coefficient_matrix.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    let expected_coefficient_matrix = Matrix::try_from([
        [-2.6786], // x^0 coefficient
        [9.50],    // x^1 coefficient
    ])
    .unwrap();

    assert_eq!(coefficient_matrix, expected_coefficient_matrix);
}

#[test]
fn quadratic_regression() {
    let data = Data::<f64>::from_csv("./tests/dataset.csv").unwrap();
    let mut coefficient_matrix = data.polynomial_regression(2).unwrap();
    // round each element
    for element in coefficient_matrix.elements_mut() {
        *element = format!("{:.4}", element).parse().unwrap();
    }

    let expected_coefficient_matrix = Matrix::try_from([
        [-34.7143], // x^0 coefficient
        [21.5476],  // x^1 coefficient
        [-1.0952],  // x^2 coefficient
    ])
    .unwrap();

    assert_eq!(coefficient_matrix, expected_coefficient_matrix);
}
