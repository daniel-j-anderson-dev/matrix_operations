use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

use crate::{Matrix, MatrixError};

impl<E> Index<usize> for Matrix<E> {
    type Output = [E];
    fn index(&self, index: usize) -> &Self::Output {
        return self.elements.index(index);
    }
}

impl<E> IndexMut<usize> for Matrix<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.elements.index_mut(index);
    }
}

impl<Element, const WIDTH: usize, const HEIGHT: usize> TryFrom<[[Element; WIDTH]; HEIGHT]>
    for Matrix<Element>
{
    type Error = MatrixError;

    /// When using array literals [Option::unwrap] or [Option::expect] are perfectly fine<br>
    /// so long as neither array dimension is zero
    /// ## Example
    /// ```rust
    /// use crate::matrix::Matrix;
    /// 
    /// let m = Matrix::try_from([
    ///     [00, 01],
    ///     [10, 11],
    /// ]).expect("Matrix dimensions are non zero");
    ///
    /// fn unknown_array_dimensions<E, const H: usize, const W: usize>(array: [[E; W]; H]) {
    ///     // W and H might be 0!!!!
    ///     let m = match Matrix::try_from(array) {
    ///         Ok(m) => m,
    ///         Err(e) => panic!("width or must must not be 0!\n{e}"),
    ///     };
    /// }
    /// ```
    fn try_from(elements: [[Element; WIDTH]; HEIGHT]) -> Result<Self, Self::Error> {
        if WIDTH == 0 || HEIGHT == 0 {
            return Err(MatrixError::InvalidDimensions);
        }
        return Ok(Matrix {
            elements: elements
                .into_iter()
                .map(|row| row.into_iter().collect())
                .collect(),
        });
    }
}

impl<E: Display> Display for Matrix<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.elements.iter() {
            for element in row.iter() {
                write!(f, "{}, ", element)?;
            }
            writeln!(f)?;
        }
        return Ok(());
    }
}

impl<E: Debug> Debug for Matrix<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.elements.iter() {
            for element in row.iter() {
                write!(f, "{:?}, ", element)?;
            }
            writeln!(f)?;
        }
        return Ok(());
    }
}
