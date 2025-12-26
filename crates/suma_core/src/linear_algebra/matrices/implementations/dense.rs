// Necesitaremos añadir 'num-traits' al Cargo.toml
use num_traits::{Zero, One, Num};
use std::fmt::{Debug, Display};

// Definimos T como genérico.
// Restricciones: Debe comportarse como número, poder copiarse,
// tener cero y uno (para algoritmos), y poder imprimirse.
#[derive(Debug, Clone, PartialEq)]
pub struct DenseMatrix<T>
where
    T: Num + Copy + Clone + Debug + Display
{
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> DenseMatrix<T>
where
    T: Num + Copy + Clone + Debug + Display
{
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        // Validación de tamaño
        assert_eq!(data.len(), rows * cols, "Data length mismatch");
        Self { data, rows, cols }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col]
    }

    // Ejemplo de por qué necesitamos el Trait Zero
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![T::zero(); rows * cols];
        Self { data, rows, cols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dense_matrix_creation() {
        let matrix = crate::matrix![
            1, 2, 3;
            4, 5, 6
        ];
        let data = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(matrix.data, data);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
    }

    #[test]
    fn test_dense_matrix_get() {
        let matrix = crate::matrix![
            1, 2;
            3, 4
        ];
        assert_eq!(matrix.get(0, 0), 1);
        assert_eq!(matrix.get(0, 1), 2);
        assert_eq!(matrix.get(1, 0), 3);
        assert_eq!(matrix.get(1, 1), 4);
    }

    #[test]
    fn test_dense_matrix_zeros() {
        let matrix: DenseMatrix<i32> = DenseMatrix::zeros(2, 3);
        assert_eq!(matrix.data, vec![0, 0, 0, 0, 0, 0]);
    }
}