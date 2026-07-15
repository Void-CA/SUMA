use std::ops::Add;
use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;

// Implementamos Add para referencias (&A + &B)
impl<'a, 'b, T> Add<&'b DenseMatrix<T>> for &'a DenseMatrix<T>
where
    T: Scalar,
{
    type Output = DenseMatrix<T>;

    fn add(self, rhs: &'b DenseMatrix<T>) -> Self::Output {
        // 1. Rigor Académico: Validación de Dimensiones
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!(
                "Error de Dimensión: No se pueden sumar matrices de {}x{} con {}x{}",
                self.rows, self.cols, rhs.rows, rhs.cols
            );
        }

        // 2. Operación Eficiente
        // Reservamos memoria una sola vez
        let mut result_data = Vec::with_capacity(self.data.len());

        // Iteramos y sumamos (usando la suma definida en el trait Scalar)
        for (a, b) in self.data.iter().zip(rhs.data.iter()) {
            result_data.push(a.clone() + b.clone());
        }

        DenseMatrix::new(self.rows, self.cols, result_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_algebra::DenseMatrix;

    #[test]
    fn test_add_two_matrices() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = DenseMatrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let result = &a + &b;
        assert_eq!(result, DenseMatrix::new(2, 2, vec![6.0, 8.0, 10.0, 12.0]));
    }

    #[test]
    fn test_add_identity() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let id = DenseMatrix::new(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
        let result = &a + &id;
        assert_eq!(result, DenseMatrix::new(2, 2, vec![2.0, 2.0, 3.0, 5.0]));
    }

    #[test]
    fn test_add_zero_matrix() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let zero = DenseMatrix::zeros(2, 2);
        let result = &a + &zero;
        assert_eq!(result, a);
    }

    #[test]
    #[should_panic(expected = "Dimensión")]
    fn test_add_dimension_mismatch() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = DenseMatrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let _ = &a + &b;
    }
}