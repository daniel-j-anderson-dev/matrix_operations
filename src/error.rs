use thiserror::Error;

use crate::Matrix;

#[derive(Debug, Error, PartialEq)]
pub enum MatrixError {
    #[error("Cannot perform {0}")]
    Arithmetic(MatrixArithmeticError),

    #[error("Cannot calculate determinant because {0}")]
    Determinant(MinorError),
}
impl MatrixError {
    /// Check if two matrices can be multiplied <br>
    /// ## Parameters
    /// - `lhs`: light hand side of a matrix product.
    /// - `rhs`: right hand side of a matrix product.
    /// ## Returns
    /// - <b>UnitType ()</b>
    ///   - if `lhs` and `rhs` can be multiplied
    /// ## Errors
    /// - [MatrixError::InvalidDimensions]
    ///   - if `lhs.width` != `rhs.height`
    pub fn multiplication<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.height() {
            Err(MatrixError::Arithmetic(MatrixArithmeticError {
                operation: MatrixOperation::Multiplication,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            }))
        } else {
            Ok(())
        };
    }

    /// Check if two matrices can be added <br>
    /// ## Parameters
    /// - `lhs`: light hand side of a matrix sum.
    /// - `rhs`: right hand side of a matrix sum.
    /// ## Returns
    /// - <b>UnitType ()</b>
    ///   - if `lhs` and `rhs` can be added
    /// ## Errors
    /// - [MatrixError::InvalidDimensions]
    ///   - if `lhs.width` != `rhs.width`
    ///   - if `lhs.height` != `rhs.height`
    pub fn addition<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
            Err(MatrixError::Arithmetic(MatrixArithmeticError {
                operation: MatrixOperation::Addition,
                lhs_width: lhs.width(),
                lhs_height: lhs.height(),
                rhs_width: rhs.width(),
                rhs_height: rhs.height(),
            }))
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// ## Parameters
    /// - `matrix`: Matrix to take a minor from.
    /// - `excluded_row_index`: the index of row to leave out form this minor.
    /// - `excluded_column_index`: the index of column to leave out form this minor.
    /// ## Returns
    /// - <b>UnitType ()</b>
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
            Err(MatrixError::Determinant(MinorError::NoSuchRow(
                excluded_row_index,
            )))
        } else if excluded_column_index >= matrix.width() {
            Err(MatrixError::Determinant(MinorError::NoSuchColumn(
                excluded_column_index,
            )))
        } else if matrix.width() != matrix.height() {
            Err(MatrixError::Determinant(MinorError::NotSquare))
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// ## Parameters
    /// - `matrix`: Matrix to take the determinate of.
    /// ## Returns
    /// - <b>UnitType ()</b>
    ///   - if there exists a determinant of `matrix`
    /// ## Errors
    /// - [MatrixError::NoDeterminant]
    ///   - if `matrix.width` != `matrix.height`
    pub fn determinant<E>(matrix: &Matrix<E>) -> Result<(), Self> {
        return if matrix.width() != matrix.height() {
            Err(MatrixError::Determinant(MinorError::NotSquare))
        } else {
            Ok(())
        };
    }

    /// Use this to check if a matrix, and index pair form a valid minor <br>
    /// ## Parameters
    /// - `matrix`: Matrix to find cofactor from.
    /// - `row_index`: index into `matrix`.
    /// - `column_index`: index into `matrix`.
    /// ## Returns
    /// - <b>UnitType ()</b>
    ///   - if there exists a determinant of `matrix`
    /// ## Errors
    /// - [MatrixError::NoCofactor]
    ///   - if `matrix.width` != `matrix.height`
    pub fn cofactor<E>(
        matrix: &Matrix<E>,
        row_index: usize,
        column_index: usize,
    ) -> Result<(), Self> {
        return if matrix.width() != matrix.height() {
            Err(MatrixError::Determinant(MinorError::NotSquare))
        } else if row_index >= matrix.height() {
            Err(MatrixError::Determinant(MinorError::NoSuchRow(row_index)))
        } else if column_index >= matrix.width() {
            Err(MatrixError::Determinant(MinorError::NoSuchColumn(
                column_index,
            )))
        } else {
            Ok(())
        };
    }
}

#[derive(Debug, PartialEq, Error)]
#[error("{operation:?} on matrices with dimensions ({lhs_height}x{lhs_width}) and ({rhs_height}x{rhs_width})")]
pub struct MatrixArithmeticError {
    operation: MatrixOperation,
    lhs_width: usize,
    lhs_height: usize,
    rhs_width: usize,
    rhs_height: usize,
}
#[derive(Debug, PartialEq)]
pub enum MatrixOperation {
    Addition,
    Multiplication,
}

#[derive(Debug, Error, PartialEq)]
pub enum MinorError {
    #[error("the row index {0} is out of bounds")]
    NoSuchRow(usize),

    #[error("the column index {0} is out of bounds")]
    NoSuchColumn(usize),

    #[error("the matrix is not square")]
    NotSquare,
}
