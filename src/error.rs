use thiserror::Error;

use crate::Matrix;

#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("Cannot perform {operation} on matrices because {dimension_error}")]
    Arithmetic {
        operation: MatrixOperation,
        dimension_error: DimensionError,
    },

    #[error("Cannot calculate determinant because {0}")]
    Minor(#[from] MinorError),

    #[error("Cannot calculate determinant because {0}")]
    Determinant(#[from] DeterminantError),

    #[error("Cannot Calculate Inverse because {0}")]
    Inverse(#[from] InverseError),
}

#[derive(Debug, Error)]
pub enum MatrixOperation {
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
    LhsWidthNotEqualToRhsHeight { 
        lhs_width: usize,
        rhs_height: usize
    },

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
                operation: MatrixOperation::Multiplication,
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
                operation: MatrixOperation::HadamardProduct,
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
    ///   - if `lhs.width` != `rhs.width`
    ///   - if `lhs.height` != `rhs.height`
    pub fn addition<E>(lhs: &Matrix<E>, rhs: &Matrix<E>) -> Result<(), Self> {
        return if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
            Err(MatrixError::Arithmetic {
                operation: MatrixOperation::Addition,
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
    ///   - if `matrix.width` != `matrix.height`
    pub fn minor<E>(
        matrix: &Matrix<E>,
        excluded_row_index: usize,
        excluded_column_index: usize,
    ) -> Result<(), Self> {
        return if excluded_row_index >= matrix.height() {
            Err(DeterminantError::MinorError(MinorError::NoSuchRow(excluded_row_index)).into())
        } else if excluded_column_index >= matrix.width() {
            Err(
                DeterminantError::MinorError(MinorError::NoSuchColumn(excluded_column_index))
                    .into(),
            )
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
    /// - [MatrixError::NoDeterminant]
    ///   - if `matrix.width` != `matrix.height`
    pub fn determinant<E>(matrix: &Matrix<E>) -> Result<(), Self> {
        return if matrix.width() == 0 || matrix.height() == 0 {
            Err(MatrixError::Determinant(
                DeterminantError::DimensionError(DimensionError::Zero),
            ))
        } else if matrix.width() != matrix.height() {
            Err(MatrixError::Determinant(
                DeterminantError::DimensionError(DimensionError::NotSquare),
            ))
        } else {
            Ok(())
        };
    }
}
