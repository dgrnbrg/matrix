//! Dense matrices.
//!
//! The storage is suitable for generic matrices.

use std::convert::Into;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use {Element, Position, Size};

/// A dense matrix.
///
/// The storage is suitable for generic matrices.
#[derive(Clone, Debug, PartialEq)]
pub struct Dense<T: Element> {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The values stored in the column-major order.
    pub values: Vec<T>,
}

matrix!(Dense);

impl<T: Element> Dense<T> {
    /// Create a matrix from a slice.
    pub fn from_slice<S: Size>(values: &[T], size: S) -> Dense<T> {
        let (rows, columns) = size.dimensions();
        debug_assert_eq!(values.len(), rows * columns);
        Dense { rows: rows, columns: columns, values: values.to_vec() }
    }

    /// Create a matrix from a vector.
    pub fn from_vec<S: Size>(values: Vec<T>, size: S) -> Dense<T> {
        let (rows, columns) = size.dimensions();
        debug_assert_eq!(values.len(), rows * columns);
        Dense { rows: rows, columns: columns, values: values }
    }
}

impl<T: Element, P: Position> Index<P> for Dense<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: P) -> &T {
        let (i, j) = index.coordinates();
        &self.values[j * self.rows + i]
    }
}

impl<T: Element, P: Position> IndexMut<P> for Dense<T> {
    #[inline]
    fn index_mut(&mut self, index: P) -> &mut T {
        let (i, j) = index.coordinates();
        &mut self.values[j * self.rows + i]
    }
}

impl<T: Element> Into<Vec<T>> for Dense<T> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.values
    }
}

impl<T: Element> Deref for Dense<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.values.deref()
    }
}

impl<T: Element> DerefMut for Dense<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.values.deref_mut()
    }
}
