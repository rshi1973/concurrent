use anyhow::{anyhow, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

#[allow(dead_code)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            rows,
            cols,
            data: data.into(),
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Debug,
{
    // Display a 2*3 matrix as {1 2 3, 4 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            write!(f, "{{")?;
            for j in 0..self.cols {
                write!(f, "{:?}", self.data[i * self.cols + j])?;

                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "}}")?;
            if i < self.rows - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix {{ rows: {}, cols: {}, data: {} }}",
            self.rows, self.cols, self
        )
    }
}

#[allow(dead_code)]
pub fn multiply_matrix<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    let mut result: Vec<T> = vec![T::default(); a.rows * b.cols];

    for i in 0..a.rows {
        for j in 0..b.cols {
            for k in 0..a.cols {
                result[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }

    Ok(Matrix {
        rows: a.rows,
        cols: b.cols,
        data: result,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() {
        let m = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", m), "{{1 2 3},{4 5 6}}");
    }

    #[test]
    fn test_multiply_matrix() {
        let a = Matrix::new(2, 3, vec![1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, vec![7, 8, 9, 10, 11, 12]);

        let result = multiply_matrix(&a, &b).unwrap();
        assert_eq!(format!("{}", result), "{{58 64},{139 154}}");
    }
}
