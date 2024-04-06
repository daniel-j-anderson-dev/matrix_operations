use std::{num::NonZeroUsize, ops::Neg};

use num::{Float, Num};

use crate::{Matrix, MatrixError, MatrixIndex};

impl<E: Num + Copy> Matrix<E> {
    pub fn transpose(&self) -> Self {
        let mut transpose = Matrix::zeros(self.width_nonzero(), self.height_nonzero());

        transpose
            .elements_mut_enumerated()
            .for_each(|(index, element)| *element = self[index.transpose()]);

        return transpose;
    }

    /// Calculate the matrix product of `self` and `rhs`. <br>
    /// The sum of dot products of `self` rows and `rhs` columns.
    /// ## Parameters
    /// - `rhs`: right hand side of product matrix.
    /// ## Returns
    /// - The product [Matrix].
    /// ## Errors
    /// - [MatrixError::Arithmetic]
    ///   - if `self.width()` != `rhs.height`
    pub fn matrix_multiply(&self, rhs: &Self) -> Result<Self, MatrixError> {
        MatrixError::multiplication(self, rhs)?;

        let mut product = Matrix::zeros(self.height_nonzero(), rhs.width_nonzero());

        for lhs_row_index in 0..self.height() {
            for rhs_col_index in 0..rhs.width() {
                let mut dot_product = E::zero();

                // I could have used rhs.height() since [MatrixError::multiplication] will return [Result::Err] if `self.width` != `rhs.height`
                for element_index in 0..self.width() {
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

    pub fn hadamard_multiply(&self, rhs: &Self) -> Result<Self, MatrixError> {
        MatrixError::hadamard_product(self, rhs)?;

        let mut product = Matrix::zeros(self.height_nonzero(), self.width_nonzero());

        for (index, product_element) in product.elements_mut_enumerated() {
            *product_element = self[index] * rhs[index];
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
        let mut product = Matrix::zeros(self.height_nonzero(), self.width_nonzero());

        for (index, product_element) in product.elements_mut_enumerated() {
            *product_element = self[index] * scalar;
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
    /// - [MatrixError::Arithmetic]
    ///   - if `self` and `rhs` are not the same dimensions
    pub fn add(&self, rhs: &Self) -> Result<Self, MatrixError> {
        MatrixError::addition(self, rhs)?;

        let mut sum = Matrix::zeros(self.height_nonzero(), self.width_nonzero());

        for (index, sum_element) in sum.elements_mut_enumerated() {
            *sum_element = self[index] + rhs[index];
        }

        return Ok(sum);
    }

    /// Constructs the minor <br>
    /// The matrix that remains after excluding a row and excluding a column.
    /// ## Returns
    /// - The [minor](https://en.wikipedia.org/wiki/Minor_(linear_algebra)) [Matrix] corresponding to `self[excluded_row_index][excluded_column_index]`.
    /// ## Errors
    /// - [MatrixError::InvalidMinor]
    ///   - if `self.width()` != `rhs.height`
    ///   - if `excluded_row_index` or `excluded_column_index` are out of bounds
    pub fn minor(&self, index: impl Into<MatrixIndex>) -> Result<Self, MatrixError> {
        let excluded_index = index.into();

        MatrixError::minor(self, excluded_index)?;

        let mut minor = Matrix::zeros(
            NonZeroUsize::new(self.height() - 1).expect("height cannot be zero"),
            NonZeroUsize::new(self.width() - 1).expect("width cannot be zero"),
        );

        let mut minor_index = MatrixIndex::from((0, 0));
        for self_row_index in 0..self.height() {
            if self_row_index == excluded_index.row() {
                continue;
            }

            minor_index.set_column(0);
            for self_column_index in 0..self.width() {
                if self_column_index == excluded_index.column() {
                    continue;
                }

                minor.set_element(minor_index, self[self_row_index][self_column_index]);

                minor_index.increment_column();
            }

            minor_index.increment_row();
        }

        return Ok(minor);
    }
}

impl<E: Num + Neg<Output = E> + Copy> Matrix<E> {
    /// Constructs the cofactor <br>
    /// <img src="https://i.imgur.com/0mAVFR3.png" width()=50% height=50%> <br>
    /// - `cofactor` == `(-1)ⁱ⁺ʲ * Mᵢⱼ`
    /// - `Mᵢⱼ` == `self.minor(i, j).determinant()`.
    /// ## Errors
    /// - [MatrixError::Determinant]
    ///   - if `self.width()` != `rhs.height`
    pub fn cofactor(&self, index: impl Into<MatrixIndex>) -> Result<E, MatrixError> {
        let index = index.into();

        let sign = if (index.row() + index.column()) % 2 == 0 {
            -E::one()
        } else {
            E::one()
        };

        let minor = self.minor(index)?;

        let minor_determinant = minor.determinant()?;

        return Ok(sign * minor_determinant);
    }

    pub fn cofactor_matrix(&self) -> Result<Self, MatrixError> {
        let mut cofactor_matrix = Matrix::zeros(self.height_nonzero(), self.width_nonzero());

        for (index, _) in self.elements_enumerated() {
            cofactor_matrix[index] = self.cofactor(index)?;
        }

        return Ok(cofactor_matrix);
    }

    /// Constructs the determinant <br>
    /// <img src="https://i.imgur.com/0mAVFR3.png" width=50% height=50%> <br>
    /// - `determinant` == `Σ(1..=n) { (-1)ⁱ⁺ʲ * Mᵢⱼ * aᵢⱼ }`
    /// - `(-1)ⁱ⁺ʲ * Mᵢⱼ` == `self.cofactor(i, j)`
    /// - `aᵢⱼ` == element at `self[i][j]`
    /// ## Returns
    /// - The determinant.
    /// ## Errors
    /// - [MatrixError::Determinant]
    ///   - if `self.width()` != `rhs.height`
    ///   - if `self.width()` OR `self.height` are `0`
    ///     - eventual i want [Matrix] to have const generic sizes with const where clauses.
    ///       This make this method only available to a [Matrix] with valid dimensions so no error is needed
    pub fn determinant(&self) -> Result<E, MatrixError> {
        MatrixError::determinant(self)?;

        if self.width() == 2 && self.height() == 2 {
            let determinant = self[0][0] * self[1][1] - self[0][1] * self[1][0];
            return Ok(determinant);
        }

        if self.width() == 1 && self.height() == 1 {
            return Ok(self[0][0]);
        }

        let mut sum = E::zero();

        const FIRST_ROW_INDEX: usize = 0;
        for column_index in 0..self.width() {
            let element = self[FIRST_ROW_INDEX][column_index];
            let cofactor = self.cofactor((FIRST_ROW_INDEX, column_index))?;
            sum = sum + (cofactor * element);
        }

        return Ok(sum);
    }
}

impl<E: Float> Matrix<E> {
    /// FIXME <br>
    /// Constructs the inverse (by matrix multiplication) <br>
    /// <img src="https://i.imgur.com/Gi79uxo.png" width=50% height=50%> <br>
    /// `C`: Cofactor Matrix <br> <img src="https://i.imgur.com/s16kLKs.png" width=50% height=50%> <br>
    /// `T`: Transpose operator
    /// `det(A)`: determinant of matrix A
    pub fn inverse(&self) -> Result<Self, MatrixError> {
        MatrixError::inverse(self)?;

        let determinant = self.determinant()?;

        let cofactor_matrix = self.cofactor_matrix()?;

        let inverse = cofactor_matrix
            .transpose()
            .scalar_multiply(E::one() / determinant);

        return Ok(inverse);
    }
}
