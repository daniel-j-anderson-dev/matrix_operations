use std::{num::NonZeroUsize, ops::Index};

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
        // type invariant
        return unsafe { NonZeroUsize::new_unchecked(self.width()) };
    }

    pub fn height(&self) -> usize {
        return self.elements.len();
    }

    /// convenience method when a [NonZeroUsize] is needed
    pub fn height_nonzero(&self) -> NonZeroUsize {
        // type invariant
        return unsafe { NonZeroUsize::new_unchecked(self.height()) };
    }

    pub fn indexes(&self) -> impl Iterator<Item = MatrixIndex> {
        let height = self.height();
        let width = self.width();

        (0..height).flat_map(move |row| (0..width).map(move |column| MatrixIndex { row, column }))
    }
}
impl<E> Matrix<E> {
    pub fn row(&self, row_index: usize) -> Option<&[E]> {
        return self.elements.get(row_index).map(|row| row.as_ref());
    }
    pub fn get_element(&self, index: impl Into<MatrixIndex>) -> Option<&E> {
        let index = index.into();
        return self
            .elements
            .get(index.row)
            .and_then(|row| row.get(index.column));
    }
    pub fn row_mut(&mut self, row_index: usize) -> Option<&mut [E]> {
        return self.elements.get_mut(row_index).map(|row| row.as_mut());
    }
    pub fn get_element_mut(&mut self, index: impl Into<MatrixIndex>) -> Option<&mut E> {
        let index = index.into();
        return self
            .elements
            .get_mut(index.row)
            .and_then(|row| row.get_mut(index.column));
    }
}
impl<E> Matrix<E> {
    pub fn column(&self, column_index: usize) -> impl Iterator<Item = &E> {
        return (0..self.height()).map(move |row_index| self.index(row_index).index(column_index));
    }
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &E>> {
        return (0..self.width()).map(|column_index| self.column(column_index));
    }
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
    pub unsafe fn zeros_unchecked(height: usize, width: usize) -> Self {
        Self {
            elements: (0..height)
                .map(|_| (0..width).map(|_| E::zero()).collect())
                .collect(),
        }
    }
    pub fn zeros(height: NonZeroUsize, width: NonZeroUsize) -> Self {
        Self {
            elements: (0..height.get())
                .map(|_| (0..width.get()).map(|_| E::zero()).collect())
                .collect(),
        }
    }
    pub fn identity(size: NonZeroUsize) -> Self {
        let mut identity = Self::zeros(size, size);

        for index in 0..size.get() {
            identity[index][index] = E::one();
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
    pub row: usize,
    pub column: usize,
}
impl MatrixIndex {
    pub fn iter(row: usize, column: usize) -> impl Iterator<Item = Self> {
        (0..row).flat_map(move |row| (0..column).map(move |column| Self { row, column }))
    }
    pub fn transpose(&self) -> Self {
        return Self {
            row: self.column,
            column: self.row,
        };
    }
    pub fn intersects(&self, other: Self) -> bool {
        self.column == other.column || self.row == other.row
    }
}
impl From<[usize; 2]> for MatrixIndex {
    /// `[row_index, column_index]`
    fn from([row, column]: [usize; 2]) -> Self {
        MatrixIndex { row, column }
    }
}
impl From<(usize, usize)> for MatrixIndex {
    /// `(row_index, column_index)`
    fn from((row, column): (usize, usize)) -> Self {
        MatrixIndex { row, column }
    }
}
