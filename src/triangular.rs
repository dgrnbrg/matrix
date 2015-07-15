use {Dense, Element, Part, Sparse};

/// A triangular matrix.
///
/// Apart from triangular matrices, the storage is suitable for symmetric and
/// Hermitian matrices. Data are stored in the [format][1] adopted by
/// [LAPACK][2].
///
/// [1]: http://www.netlib.org/lapack/lug/node123.html
/// [2]: http://www.netlib.org/lapack
#[derive(Clone, Debug, PartialEq)]
pub struct Triangular<T: Element> {
    /// The number of rows or columns.
    pub size: usize,
    /// The storage format.
    pub format: Part,
    /// The values stored in the column-major order.
    pub values: Vec<T>,
}

macro_rules! debug_valid(
    ($matrix:ident) => (debug_assert!(
        $matrix.values.len() == $matrix.size * ($matrix.size + 1) / 2
    ));
);

matrix!(Triangular, size, size);

impl<T: Element> Sparse for Triangular<T> {
    #[inline]
    fn nonzeros(&self) -> usize {
        self.size * (self.size + 1) / 2
    }
}

impl<'l, T: Element> From<&'l Triangular<T>> for Dense<T> {
    fn from(matrix: &'l Triangular<T>) -> Dense<T> {
        debug_valid!(matrix);

        let &Triangular { size, format, ref values } = matrix;

        let mut matrix = Dense {
            rows: size,
            columns: size,
            values: vec![T::zero(); size * size],
        };

        match format {
            Part::Lower => {
                let mut k = 0;
                for j in 0..size {
                    for i in j..size {
                        matrix.values[j * size + i] = values[k];
                        k += 1;
                    }
                }
            },
            Part::Upper => {
                let mut k = 0;
                for j in 0..size {
                    for i in 0..(j + 1) {
                        matrix.values[j * size + i] = values[k];
                        k += 1;
                    }
                }
            },
        }

        matrix
    }
}

impl<T: Element> From<Triangular<T>> for Dense<T> {
    fn from(matrix: Triangular<T>) -> Dense<T> {
        (&matrix).into()
    }
}

#[cfg(test)]
mod tests {
    use {Dense, Part, Triangular};

    #[test]
    fn into_dense_lower() {
        let matrix = Triangular {
            size: 4,
            format: Part::Lower,
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: Dense<_> = matrix.into();

        assert_eq!(&matrix[..], &[
            1.0, 2.0, 3.0,  4.0,
            0.0, 5.0, 6.0,  7.0,
            0.0, 0.0, 8.0,  9.0,
            0.0, 0.0, 0.0, 10.0,
        ]);
    }

    #[test]
    fn into_dense_upper() {
        let matrix = Triangular {
            size: 4,
            format: Part::Upper,
            values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let matrix: Dense<_> = matrix.into();

        assert_eq!(&matrix[..], &[
            1.0, 0.0, 0.0,  0.0,
            2.0, 3.0, 0.0,  0.0,
            4.0, 5.0, 6.0,  0.0,
            7.0, 8.0, 9.0, 10.0,
        ]);
    }
}
