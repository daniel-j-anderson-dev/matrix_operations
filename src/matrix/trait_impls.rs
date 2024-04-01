use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::Matrix;

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

impl<E: Copy, const W: usize, const H: usize> From<[[E; W]; H]> for Matrix<E> {
    fn from(value: [[E; W]; H]) -> Self {
        return Matrix {
            elements: value
                .iter()
                .map(|row| row.iter().cloned().collect::<Box<[E]>>())
                .collect(),
            width: W,
            height: H,
        };
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
