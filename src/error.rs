use thiserror::Error;

use crate::Matrix;

#[derive(Debug, Error, PartialEq)]
pub enum MatrixError {
    #[error("Cannot perform {operation:?} on matrices with dimensions ({lhs_height}x{lhs_width}) and ({rhs_height}x{rhs_width})")]
    InvalidDimensions {
        operation: MatrixOperation,
        lhs_width: usize,
        lhs_height: usize,
        rhs_width: usize,
        rhs_height: usize,
    },

    #[error("There is no minor because {0}")]
    InvalidMinor(MatrixMinorError),
}
impl MatrixError {
    /// Check if two matrices can be multiplied <br>
    /// Ensures that
    /// - `lhs.width` == `rhs.height`
    pub fn multiplication<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() == rhs.height() {
            Ok(())
        } else {
            Err(MatrixError::InvalidDimensions {
                operation: MatrixOperation::Multiplication,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            })
        };
    }
    /// Check if two matrices can be added <br>
    /// Ensures that
    /// - `lhs.width` == `rhs.width`
    /// - `lhs.height` == `rhs.height`
    pub fn addition<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() == rhs.width() || lhs.height() == rhs.height() {
            Ok(())
        } else {
            Err(MatrixError::InvalidDimensions {
                operation: MatrixOperation::Addition,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            })
        };
    }
    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// Ensures that
    /// - `excluded_row_index` is valid
    /// - `excluded_column_index` is valid
    /// - `matrix.width` == `matrix.height`
    pub fn minor<E>(
        matrix: &Matrix<E>,
        excluded_row_index: usize,
        excluded_column_index: usize,
    ) -> Result<(), Self> {
        return if excluded_row_index >= matrix.height() {
            Err(MatrixError::InvalidMinor(MatrixMinorError::NoSuchRow(
                excluded_row_index,
            )))
        } else if excluded_column_index >= matrix.width() {
            Err(MatrixError::InvalidMinor(MatrixMinorError::NoSuchColumn(
                excluded_column_index,
            )))
        } else if matrix.width() != matrix.height() {
            Err(MatrixError::InvalidMinor(MatrixMinorError::NotSquare))
        } else {
            Ok(())
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum MatrixOperation {
    Addition,
    Multiplication,
}

#[derive(Debug, Error, PartialEq)]
pub enum MatrixMinorError {
    #[error("the matrix is not square")]
    NotSquare,
    #[error("the row index {0} is out of bounds")]
    NoSuchRow(usize),
    #[error("the column index {0} is out of bounds")]
    NoSuchColumn(usize),
}