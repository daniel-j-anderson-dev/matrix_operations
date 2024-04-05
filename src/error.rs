use std::ops::Neg;

use num::Num;
use thiserror::Error;

use crate::{matrix::MatrixIndex, Matrix};

#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("Cannot perform {operation} on matrices because {dimension_error}")]
    Arithmetic {
        operation: MatrixArithmeticOperation,
        dimension_error: DimensionError,
    },

    #[error("Cannot calculate determinant because {0}")]
    Minor(#[from] MinorError),

    #[error("Cannot calculate determinant because {0}")]
    Determinant(#[from] DeterminantError),

    #[error("Cannot Calculate Inverse because {0}")]
    Inverse(#[from] InverseError),

    #[error("Cannot create matrix because {0}")]
    DimensionError(#[from] DimensionError),
}
impl MatrixError {
    /// Check if two matrices can be multiplied <br>
    /// ## Parameters
    /// - `lhs`: light hand side of a matrix product.
    /// - `rhs`: right hand side of a matrix product.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if `lhs` and `rhs` can be multiplied
    /// ## Errors
    /// - [MatrixError::Arithmetic]
    ///   - if `lhs.width` != `rhs.height`
    pub fn multiplication<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.height() {
            Err(MatrixError::Arithmetic {
                operation: MatrixArithmeticOperation::Multiplication,
                dimension_error: DimensionError::LhsWidthNotEqualToRhsHeight {
                    lhs_width: lhs.width(),
                    rhs_height: rhs.height(),
                },
            })
        } else {
            Ok(())
        };
    }

    /// Check if two matrices can be element-wise multiplied <br>
    /// ## Parameters
    /// - `lhs`: light hand side of an element-wise matrix product.
    /// - `rhs`: right hand side of an element-wise matrix product.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if `lhs` and `rhs` can be element-wise multiplied
    /// ## Errors
    /// - [MatrixError::Arithmetic]
    ///   - if `lhs` and `rhs` have different dimensions
    pub fn hadamard_product<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
            Err(MatrixError::Arithmetic {
                operation: MatrixArithmeticOperation::HadamardProduct,
                dimension_error: DimensionError::DifferentDimensions {
                    lhs_width: lhs.width(),
                    lhs_height: lhs.height(),
                    rhs_width: rhs.width(),
                    rhs_height: rhs.height(),
                },
            })
        } else {
            Ok(())
        };
    }

    /// Check if two matrices can be added <br>
    /// ## Parameters
    /// - `lhs`: light hand side of a matrix sum.
    /// - `rhs`: right hand side of a matrix sum.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if `lhs` and `rhs` can be added
    /// ## Errors
    /// - [MatrixError::Arithmetic]
    ///   - if `lhs` and `rhs` have different dimensions
    pub fn addition<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
            Err(MatrixError::Arithmetic {
                operation: MatrixArithmeticOperation::Addition,
                dimension_error: DimensionError::DifferentDimensions {
                    lhs_width: lhs.width(),
                    lhs_height: lhs.height(),
                    rhs_width: rhs.width(),
                    rhs_height: rhs.height(),
                },
            })
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// ## Parameters
    /// - `matrix`: Matrix to take a minor from.
    /// - `excluded_row_index`: the index of the row to leave out form this minor.
    /// - `excluded_column_index`: the index of the column to leave out form this minor.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if there exists a minor of `matrix` at the corresponding indexes
    /// ## Errors
    /// - [MatrixError::InvalidDimensions]
    ///   - if `excluded_row_index` >= `matrix.height`
    ///   - if `excluded_column_index` >= `matrix.width`
    ///   - if `matrix` is not square
    pub fn minor<E>(matrix: &Matrix<E>, index: impl Into<MatrixIndex>) -> Result<(), Self> {
        let index = index.into();

        return if index.row() >= matrix.height() {
            Err(DeterminantError::MinorError(MinorError::NoSuchRow(index.row())).into())
        } else if index.column() >= matrix.width() {
            Err(DeterminantError::MinorError(MinorError::NoSuchColumn(index.column())).into())
        } else if matrix.width() == 0 || matrix.height() == 0 {
            Err(DeterminantError::DimensionError(DimensionError::Zero).into())
        } else if matrix.width() < 2 || matrix.height() < 2 {
            Err(DeterminantError::DimensionError(DimensionError::TooSmall).into())
        } else if matrix.width() != matrix.height() {
            Err(DeterminantError::DimensionError(DimensionError::NotSquare).into())
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// ## Parameters
    /// - `matrix`: Matrix to take the determinate of.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if there exists a determinant of `matrix`
    /// ## Errors
    /// - [MatrixError::Determinant]
    ///   - if `matrix.width` != `matrix.height`
    ///   - if either dimension of `matrix` is `0`
    pub fn determinant<E>(matrix: &Matrix<E>) -> Result<(), Self> {
        return if matrix.width() == 0 || matrix.height() == 0 {
            Err(MatrixError::Determinant(DeterminantError::DimensionError(
                DimensionError::Zero,
            )))
        } else if matrix.width() != matrix.height() {
            Err(MatrixError::Determinant(DeterminantError::DimensionError(
                DimensionError::NotSquare,
            )))
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix is invertible <br>
    /// (in terms of matrix multiplication)
    /// ## Parameters
    /// - `matrix`: Matrix to invert.
    /// ## Returns
    /// - <b>UnitType `()`</b>
    ///   - if there exists a multiplicative inverse of `matrix`
    /// ## Errors
    /// - [MatrixError::Inverse]
    ///   - if either dimension of `matrix` is `0`
    ///   - if `matrix` is not square
    ///   - if the determinant of `matrix` is `0`
    pub fn inverse<E: Num + Neg<Output = E> + Copy>(matrix: &Matrix<E>) -> Result<(), Self> {
        return if matrix.width() == 0 || matrix.height() == 0 {
            Err(MatrixError::Inverse(InverseError::DimensionError(
                DimensionError::Zero,
            )))
        } else if matrix.width() != matrix.height() {
            Err(MatrixError::Inverse(InverseError::DimensionError(
                DimensionError::NotSquare,
            )))
        } else if matrix.determinant()?.is_zero() {
            Err(MatrixError::Inverse(InverseError::DeterminantZero))
        } else {
            Ok(())
        };
    }
}

#[derive(Debug, Error)]
pub enum MatrixArithmeticOperation {
    #[error("Matrix Addition")]
    Addition,
    #[error("Matrix Multiplication")]
    Multiplication,
    #[error("Hadamard product (Element-wise multiplication)")]
    HadamardProduct,
}

#[derive(Debug, Error)]
pub enum DimensionError {
    #[error("the matrices are not the same size (lhs: {lhs_height}x{lhs_width}, rhs: {rhs_height}x{rhs_width})")]
    DifferentDimensions {
        lhs_width: usize,
        lhs_height: usize,
        rhs_width: usize,
        rhs_height: usize,
    },

    #[error("the width of lhs matrix does not equal the height of rhs matrix (lhs width: {lhs_width}, rhs height: {rhs_height})")]
    LhsWidthNotEqualToRhsHeight { lhs_width: usize, rhs_height: usize },

    #[error("the matrix is not square")]
    NotSquare,

    #[error("the matrix is too small")]
    TooSmall,

    #[error("the matrix has 0 size")]
    Zero,
}

#[derive(Debug, Error)]
pub enum MinorError {
    #[error("The Minor does not exist at row index {0} is out of bounds")]
    NoSuchRow(usize),

    #[error("The Minor does not exist at column index {0} is out of bounds")]
    NoSuchColumn(usize),

    #[error("The Minor does not exist because {0}")]
    InvalidDimensions(#[from] DimensionError),
}

#[derive(Debug, Error)]
pub enum DeterminantError {
    #[error("{0}")]
    MinorError(#[from] MinorError),

    #[error("The Determinant does not exist because {0}")]
    DimensionError(#[from] DimensionError),
}

#[derive(Debug, Error)]
pub enum InverseError {
    #[error("{0}")]
    DeterminantError(#[from] DeterminantError),

    #[error("The determinant is 0")]
    DeterminantZero,

    #[error("{0}")]
    DimensionError(#[from] DimensionError),
}
