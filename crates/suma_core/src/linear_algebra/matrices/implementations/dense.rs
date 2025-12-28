// src/linear_algebra/matrices/implementations/dense.rs
use crate::linear_algebra::traits::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct DenseMatrix<T>
where
    T: Scalar 
{
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> DenseMatrix<T>
where
    T: Scalar
{
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length mismatch");
        Self { data, rows, cols }
    }

    // Cambio importante: Ahora devuelve T clonándolo, porque T podría no ser Copy (Expr)
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col].clone() 
    }

    // Opcional: Una versión más eficiente que devuelve referencia
    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        // T::zero() viene de nuestro trait Zero
        // vec! con clone requiere que T sea Clone, lo cual Scalar garantiza.
        let data = vec![T::zero(); rows * cols]; 
        Self { data, rows, cols }
    }
}
