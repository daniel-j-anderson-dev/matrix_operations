use num::Num;

#[derive(Clone, PartialEq)]
pub struct Matrix<ELEMENT> {
    elements: Box<[Box<[ELEMENT]>]>,
}
impl<E> Matrix<E> {
    pub fn width(&self) -> usize {
        return self.elements[0].len();
    }
    pub fn height(&self) -> usize {
        return self.elements.len();
    }
    pub fn rows(&self) -> impl Iterator<Item = &[E]> {
        return self.elements.iter().map(Box::as_ref);
    }
    pub fn elements(&self) -> impl Iterator<Item = &E> {
        return self.elements.iter().flat_map(|row| row.iter());
    }
}
impl<E: Num + Copy> Matrix<E> {
    pub fn zeros(width: usize, height: usize) -> Self {
        return Matrix {
            elements: vec![vec![E::zero(); width].into_boxed_slice(); height].into_boxed_slice(),
        };
    }
    pub fn identity(width: usize, height: usize) -> Self {
        let mut identity = Self::zeros(width, height);

        for row_index in 0..height {
            for column_index in 0..width {
                if row_index == column_index {
                    identity[row_index][column_index] = E::one();
                }
            }
        }

        return identity;
    }
}

pub mod operations;
pub mod trait_impls;
