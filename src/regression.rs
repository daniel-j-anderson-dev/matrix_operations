use std::{fmt::Debug, num::NonZeroUsize, ops::Index};

use num::Float;

use crate::{DataSet, Matrix, MatrixError};

pub trait Regression<T> {
    type Error;
    fn polynomial_regression(&self, degree: usize) -> Result<Matrix<T>, Self::Error>;
}

impl<F: Float> DataSet<F> {
    pub fn polynomial_input_matrix(&self, degree: usize) -> Matrix<F> {
        let width = NonZeroUsize::new(degree + 1).expect("usize + 1 is always >");
        let height = self.len_nonzero();
    
        let mut input_matrix = Matrix::zeros(height, width);
    
        for column_index in 0..=degree {
            let exponent = (degree - column_index) as i32;
    
            for row_index in 0..height.get() {
                let input_value = self
                    .data()
                    .index(row_index)
                    .input();
    
                let input_matrix_value = input_value.powi(exponent);
    
                input_matrix[row_index][column_index] = input_matrix_value;
            }
        }
        return input_matrix;
    }

    pub fn polynomial_output_matrix(&self,) -> Matrix<F> {
        /// This is safe because `1` is a valid value of [NonZeroUsize]
        const ONE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(1) };

        let height = self.len_nonzero();
        let mut output_matrix = Matrix::zeros(height, ONE);

        for row_index in 0..height.get() {
            let output_value = self.data().index(row_index).output();

            output_matrix[row_index][0] = *output_value;
        }

        return output_matrix;
    }
}

impl<F: Float + Debug> Regression<F> for DataSet<F> {
    type Error = MatrixError;
    fn polynomial_regression(&self, degree: usize) -> Result<Matrix<F>, Self::Error> {
        let input_matrix = self.polynomial_input_matrix(degree);
        let output_matrix = self.polynomial_output_matrix();

        let input_transpose = input_matrix.transpose();

        let pseudo_inverse = input_transpose
            .matrix_multiply(&input_matrix)?
            .inverse()?
            .matrix_multiply(&input_transpose)?;

        let coefficient_matrix = pseudo_inverse.matrix_multiply(&output_matrix)?;

        dbg!(&input_matrix, &output_matrix, &coefficient_matrix);

        return Ok(coefficient_matrix);
    }
}
