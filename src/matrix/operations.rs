use std::ops::Neg;

use num::Num;

use crate::{Matrix, MatrixError};

impl<E: Num + Copy> Matrix<E> {
    /// Calculate the matrix product of `self` and `rhs`. <br>
    /// element wise dot product of `self`'s rows and `rhs`'s columns.
    /// ## Parameters
    /// - `rhs`: right hand side of product matrix.
    /// ## Returns
    /// - The product [Matrix].
    /// ## Errors
    /// - [MatrixError::InvalidDimensions]
    ///   - if `self.width` != `rhs.height`
    pub fn matrix_multiply(&self, rhs: &Self) -> Result<Self, MatrixError> {
        MatrixError::multiplication(self, rhs)?;

        let mut product = Matrix::zeros(rhs.width, self.height);

        for lhs_row_index in 0..self.height {
            for rhs_col_index in 0..rhs.width {
                let mut dot_product = E::zero();

                for element_index in 0..self.width {
                    let lhs_element = self[lhs_row_index][element_index];
                    let rhs_element = rhs[element_index][rhs_col_index];

                    let element_product = lhs_element * rhs_element;

                    dot_product = dot_product + element_product;
                }

                product[lhs_row_index][rhs_col_index] = dot_product;
            }
        }

        return Ok(product);
    }

    /// Calculate the scalar product of `self` and `scalar`. <br>
    /// each element of `self` is multiplied by `scalar`.
    /// ## Parameters
    /// - `scalar`: a scalar value to be multiplied.
    /// ## Returns
    /// - The scalar product [Matrix].
    pub fn scalar_multiply(&self, scalar: E) -> Self {
        let mut product = Self::zeros(self.width, self.height);

        for row_index in 0..self.height {
            for column_index in 0..self.width {
                product[row_index][column_index] = self[row_index][column_index] * scalar;
            }
        }

        return product;
    }

    /// Calculate the matrix sum of `self` and `rhs`. <br>
    /// element wise addition of `self` and `rhs`.
    /// ## Parameters
    /// - `rhs`: right hand side of sum matrix.
    /// ## Returns
    /// - The sum [Matrix].
    /// ## Errors
    /// - [MatrixError::InvalidDimensions]
    ///   - if `self` and `rhs` are not the same dimensions
    pub fn add(&self, rhs: &Self) -> Result<Self, MatrixError> {
        MatrixError::addition(self, rhs)?;

        let mut sum = Self::zeros(self.width, self.height);

        for row_index in 0..self.height {
            for column_index in 0..self.width {
                sum[row_index][column_index] =
                    self[row_index][column_index] + rhs[row_index][column_index];
            }
        }

        return Ok(sum);
    }

    /// Constructs the minor <br>
    /// The matrix that remains after excluding a row and excluding a column.
    /// ## Returns
    /// - The [minor](https://en.wikipedia.org/wiki/Minor_(linear_algebra)) [Matrix] corresponding to `self[excluded_row_index][excluded_column_index]`.
    /// ## Errors
    /// - [MatrixError::InvalidMinor]
    ///   - if `self.width` != `rhs.height`
    ///   - if `excluded_row_index` or `excluded_column_index` are out of bounds
    pub fn minor(
        &self,
        excluded_row_index: usize,
        excluded_column_index: usize,
    ) -> Result<Self, MatrixError> {
        MatrixError::minor(self, excluded_row_index, excluded_column_index)?;

        let mut minor = Self::zeros(self.width - 1, self.height - 1);

        let mut minor_row_index = 0;
        for self_row_index in 0..self.height {
            if self_row_index == excluded_row_index {
                continue;
            }

            let mut minor_column_index = 0;
            for self_column_index in 0..self.width {
                if self_column_index == excluded_column_index {
                    continue;
                }

                if let Some(minor_element) = minor
                    .elements
                    .get_mut(minor_row_index)
                    .and_then(|row| row.get_mut(minor_column_index))
                {
                    *minor_element = self[self_row_index][self_column_index];
                }

                minor_column_index += 1;
            }

            minor_row_index += 1;
        }

        return Ok(minor);
    }
}

impl<E: Num + Neg<Output = E> + Copy> Matrix<E> {
    /// Constructs the determinant <br>
    /// <img src="https://i.imgur.com/0mAVFR3.png" width=50% height=50%> <br>
    /// cofactor == `(-1)ⁱ+ʲ * Mᵢⱼ`
    /// ## Returns
    /// - The determinant.
    /// ## Errors
    /// - [MatrixError::NoDeterminant]
    ///   - if `self.width` != `rhs.height`
    pub fn cofactor(&self, row_index: usize, column_index: usize) -> Result<E, MatrixError> {
        let sign = if row_index + column_index % 2 == 0 {
            -E::one()
        } else {
            E::one()
        };

        return Ok(self
            .minor(row_index, column_index)?
            .scalar_multiply(sign)
            .determinant()?);
    }

    /// Constructs the determinant <br>
    /// <img src="https://i.imgur.com/0mAVFR3.png" width=50% height=50%> <br>
    /// - `i`: row_index
    /// - `j`: column_index
    /// - `Mᵢⱼ`: `self.minor(i, j).determinant()`.
    /// - `(-1)ⁱ+ʲ * Mᵢⱼ`: `self.cofactor(i, j)`
    /// ## Returns
    /// - The determinant.
    /// ## Errors
    /// - [MatrixError::NoDeterminant]
    ///   - if `self.width` != `rhs.height`
    pub fn determinant(&self) -> Result<E, MatrixError> {
        MatrixError::determinant(self)?;

        let mut sum = E::zero();

        for row_index in 0..self.height {
            for column_index in 0..self.width {
                let cofactor = self.cofactor(row_index, column_index)?;
                sum = sum + (cofactor * self[row_index][column_index]);
            }
        }

        return Ok(sum);
    }
}
