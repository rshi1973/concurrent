use anyhow::Result;
use current::{multiply_matrix, Matrix};

fn main() -> Result<()> {
    let a = Matrix {
        rows: 2,
        cols: 3,
        data: vec![1, 2, 3, 4, 5, 6],
    };

    let b = Matrix {
        rows: 3,
        cols: 2,
        data: vec![1, 2, 3, 4, 5, 6],
    };

    let c = multiply_matrix(&a, &b)?;
    println!("{:?}", c);

    Ok(())
}
