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
        return NonZeroUsize::new(self.elements[0].len()).expect("width cannot be zero");
    }
    pub fn height(&self) -> usize {
        return self.elements.len();
    }
    /// convenience method when a [NonZeroUsize] is needed
    pub fn height_nonzero(&self) -> NonZeroUsize {
        return NonZeroUsize::new(self.elements.len()).expect("height cannot be zero");
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
