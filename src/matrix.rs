use crate::vector::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

const NUM_THREADS: usize = 4;

#[allow(dead_code)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
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

impl<T> Mul for Matrix<T>
where
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    type Output = Result<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply_matrix(&self, &rhs)
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
    T: fmt::Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    // generate four threads to do the dot_product.
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let mut result: Vec<T> = vec![T::default(); a.rows * b.cols];
    let mut receivers = Vec::with_capacity(a.rows * b.cols);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col = Vector::new(
                b.data[j..b.cols * b.rows]
                    .iter()
                    .step_by(b.cols)
                    .cloned()
                    .collect::<Vec<T>>(),
            );
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Error: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        result[output.idx] = output.value;
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
