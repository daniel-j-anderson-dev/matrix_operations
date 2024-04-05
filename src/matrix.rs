use std::num::NonZeroUsize;

use num::Num;

#[derive(Clone, PartialEq)]
pub struct Matrix<ELEMENT> {
    elements: Box<[Box<[ELEMENT]>]>,
}
impl<E> Matrix<E> {
    pub fn width(&self) -> usize {
        return self.elements[0].len();
    }
    /// convenience method when a [NonZeroUsize] is needed
    pub fn width_nonzero(&self) -> NonZeroUsize {
        return NonZeroUsize::new(self.width()).expect("width cannot be zero");
    }
    pub fn height(&self) -> usize {
        return self.elements.len();
    }
    /// convenience method when a [NonZeroUsize] is needed
    pub fn height_nonzero(&self) -> NonZeroUsize {
        return NonZeroUsize::new(self.height()).expect("height cannot be zero");
    }
}
impl<E> Matrix<E> {
    pub fn get_row(&self, index: MatrixIndex) -> Option<&[E]> {
        return self.elements.get(index.row()).map(|row| row.as_ref());
    }
    pub fn get_element(&self, index: impl Into<MatrixIndex>) -> Option<&E> {
        let index = index.into();
        return self
            .elements
            .get(index.row())
            .and_then(|row| row.get(index.column()));
    }
    /// Set the `self[index] = value` if index is valid
    pub fn set_element(&mut self, index: impl Into<MatrixIndex>, value: E) -> Option<()> {
        return self.get_element_mut(index).map(|element| *element = value);
    }
    pub fn get_row_mut(&mut self, index: impl Into<MatrixIndex>) -> Option<&mut [E]> {
        let index = index.into();
        return self.elements.get_mut(index.row()).map(|row| row.as_mut());
    }
    pub fn get_element_mut(&mut self, index: impl Into<MatrixIndex>) -> Option<&mut E> {
        let index = index.into();
        return self
            .elements
            .get_mut(index.row())
            .and_then(|row| row.get_mut(index.column()));
    }
}
impl<E> Matrix<E> {
    pub fn rows(&self) -> impl Iterator<Item = &[E]> {
        return self.elements.iter().map(Box::as_ref);
    }
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [E]> {
        return self.elements.iter_mut().map(Box::as_mut);
    }
    pub fn elements(&self) -> impl Iterator<Item = &E> {
        return self.elements.iter().flat_map(|row| row.iter());
    }
    pub fn elements_mut(&mut self) -> impl Iterator<Item = &mut E> {
        return self.elements.iter_mut().flat_map(|row| row.iter_mut());
    }
    pub fn elements_enumerated(&self) -> impl Iterator<Item = (MatrixIndex, &E)> {
        return self
            .elements
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(column_index, element)| ((row_index, column_index).into(), element))
            });
    }
    pub fn elements_mut_enumerated(&mut self) -> impl Iterator<Item = (MatrixIndex, &mut E)> {
        return self
            .elements
            .iter_mut()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter_mut()
                    .enumerate()
                    .map(move |(column_index, element)| ((row_index, column_index).into(), element))
            });
    }
}
impl<E: Num + Copy> Matrix<E> {
    pub fn zeros(height: NonZeroUsize, width: NonZeroUsize) -> Self {
        return Matrix {
            elements: vec![vec![E::zero(); width.get()].into_boxed_slice(); height.get()]
                .into_boxed_slice(),
        };
    }
    pub fn identity(height: NonZeroUsize, width: NonZeroUsize) -> Self {
        let mut identity = Self::zeros(width, height);

        for row_index in 0..height.get() {
            for column_index in 0..width.get() {
                if row_index == column_index {
                    identity[row_index][column_index] = E::one();
                }
            }
        }

        return identity;
    }
    pub fn set_zero(&mut self) {
        self.elements_mut().for_each(|element| *element = E::zero());
    }
}

pub mod operations;
pub mod trait_impls;

/// `MatrixIndex(row_index, column_index)`
#[derive(Debug, Clone, Copy)]
pub struct MatrixIndex {
    row: usize,
    column: usize,
}
impl MatrixIndex {
    pub fn row(&self) -> usize {
        return self.row;
    }
    pub fn column(&self) -> usize {
        return self.column;
    }

    /// Add one to the row index
    pub fn increment_row(&mut self) {
        self.row += 1;
    }

    /// Add one to the column index
    pub fn increment_column(&mut self) {
        self.column += 1;
    }

    pub fn set_row(&mut self, row_index: usize) {
        self.row = row_index;
    }

    pub fn set_column(&mut self, column_index: usize) {
        self.column = column_index;
    }

    pub fn transpose(&self) -> Self {
        return Self {
            row: self.column,
            column: self.row,
        };
    }
}
impl From<(usize, usize)> for MatrixIndex {
    /// `(row_index, column_index)`
    fn from(value: (usize, usize)) -> Self {
        return MatrixIndex {
            row: value.0,
            column: value.1,
        };
    }
}
