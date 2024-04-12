pub mod data_set;
pub mod error;
pub mod matrix;
pub mod regression;
#[cfg(test)]
pub mod test;

pub use crate::{data_set::*, error::*, matrix::*, regression::*};
