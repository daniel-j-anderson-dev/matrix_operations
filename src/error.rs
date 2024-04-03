use thiserror::Error;

use crate::Matrix;

#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("{0}")]
    Arithmetic(#[from] MatrixArithmeticError),

    #[error("Cannot calculate determinant because {0}")]
    Determinant(#[from] MatrixMinorError),

    #[error("Cannot create a matrix with 0 rows or columns")]
    InvalidDimensions,
}

#[derive(Debug, Error)]
#[error("Cannot perform {operation} on matrices with dimensions ({lhs_height}x{lhs_width}) and ({rhs_height}x{rhs_width})")]
pub struct MatrixArithmeticError {
    operation: MatrixOperation,
    lhs_width: usize,
    lhs_height: usize,
    rhs_width: usize,
    rhs_height: usize,
}
#[derive(Debug, Error)]
pub enum MatrixOperation {
    #[error("Matrix Addition")]
    Addition,
    #[error("Matrix Multiplication")]
    Multiplication,
}

#[derive(Debug, Error)]
pub enum MatrixMinorError {
    #[error("the row index {0} is out of bounds")]
    NoSuchRow(usize),

    #[error("the column index {0} is out of bounds")]
    NoSuchColumn(usize),

    #[error("the matrix is not square")]
    NotSquare,

    #[error("the matrix is too small or empty")]
    TooSmall,
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
            Err(MatrixArithmeticError {
                operation: MatrixOperation::Multiplication,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            }
            .into())
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
    ///   - if `lhs.width` != `rhs.width`
    ///   - if `lhs.height` != `rhs.height`
    pub fn addition<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
            Err(MatrixArithmeticError {
                operation: MatrixOperation::Addition,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            }
            .into())
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
    ///   - if `matrix.width` != `matrix.height`
    pub fn minor<E>(
        matrix: &Matrix<E>,
        excluded_row_index: usize,
        excluded_column_index: usize,
    ) -> Result<(), Self> {
        return if excluded_row_index >= matrix.height() {
            Err(MatrixMinorError::NoSuchRow(excluded_row_index).into())
        } else if excluded_column_index >= matrix.width() {
            Err(MatrixMinorError::NoSuchColumn(excluded_column_index).into())
        } else if matrix.width() < 2 || matrix.height() < 2 {
            Err(MatrixMinorError::TooSmall.into())
        } else if matrix.width() != matrix.height() {
            Err(MatrixMinorError::NotSquare.into())
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
    /// - [MatrixError::NoDeterminant]
    ///   - if `matrix.width` != `matrix.height`
    pub fn determinant<E>(matrix: &Matrix<E>) -> Result<(), Self> {
        return if matrix.width() == 0 || matrix.height() == 0 {
            Err(MatrixMinorError::TooSmall.into())
        } else if matrix.width() != matrix.height() {
            Err(MatrixMinorError::NotSquare.into())
        } else {
            Ok(())
        };
    }
}
