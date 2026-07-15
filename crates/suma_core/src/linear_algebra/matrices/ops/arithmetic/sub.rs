use std::ops::Sub;
use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;

// Implementamos Sub para referencias (&A + &B)
impl<'a, 'b, T> Sub<&'b DenseMatrix<T>> for &'a DenseMatrix<T>
where
    T: Scalar,
{
    type Output = DenseMatrix<T>;

    fn sub(self, rhs: &'b DenseMatrix<T>) -> Self::Output {
        // 1. Rigor Académico: Validación de Dimensiones
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!(
                "Error de Dimensión: No se pueden restar matrices de {}x{} con {}x{}",
                self.rows, self.cols, rhs.rows, rhs.cols
            );
        }

        // 2. Operación Eficiente
        // Reservamos memoria una sola vez
        let mut result_data = Vec::with_capacity(self.data.len());

        // Iteramos y restamos (usando la resta definida en el trait Scalar)
        for (a, b) in self.data.iter().zip(rhs.data.iter()) {
            result_data.push(a.clone() - b.clone());
        }

        DenseMatrix::new(self.rows, self.cols, result_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_algebra::DenseMatrix;

    #[test]
    fn test_sub_two_matrices() {
        let a = DenseMatrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let b = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = &a - &b;
        assert_eq!(result, DenseMatrix::new(2, 2, vec![4.0, 4.0, 4.0, 4.0]));
    }

    #[test]
    fn test_sub_self_is_zero() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = &a - &a;
        assert_eq!(result, DenseMatrix::zeros(2, 2));
    }
}