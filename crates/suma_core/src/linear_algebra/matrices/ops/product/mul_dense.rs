use std::ops::Mul;
use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError; // Importamos el error

impl<'a, 'b, T> Mul<&'b DenseMatrix<T>> for &'a DenseMatrix<T>
where
    T: Scalar,
{
    // CAMBIO IMPORTANTE: El Output ahora es un Result
    type Output = Result<DenseMatrix<T>, LinearAlgebraError>;

    fn mul(self, rhs: &'b DenseMatrix<T>) -> Self::Output {
        // Validación segura sin panic
        if self.cols != rhs.rows {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Matrix Multiplication (Cols A vs Rows B)".to_string(),
                expected: self.cols,
                found: rhs.rows,
            });
        }

        let m = self.rows;
        let n = self.cols;
        let p = rhs.cols;

        let mut result_data = Vec::with_capacity(m * p);

        // Algoritmo O(n^3) (Igual que antes)
        for i in 0..m {
            for j in 0..p {
                let mut sum = T::zero();
                for k in 0..n {
                    let a_ik = self.get(i, k);
                    let b_kj = rhs.get(k, j);
                    sum = sum + (a_ik * b_kj);
                }
                result_data.push(sum);
            }
        }

        // Devolvemos Ok(Matriz)
        Ok(DenseMatrix::new(m, p, result_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
    use crate::linear_algebra::error::LinearAlgebraError;

    #[test]
    fn test_mul_numeric_2x2() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let b = DenseMatrix::new(2, 2, vec![2.0, 0.0, 1.0, 2.0]);
        let expected = DenseMatrix::new(2, 2, vec![4.0, 4.0, 10.0, 8.0]);

        // Note el uso de unwrap() o '?' en los tests porque esperamos éxito
        let result = (&a * &b).expect("Multiplicación válida falló");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_dimension_mismatch_error() {
        // 2x3 * 2x2 -> Error (Cols A=3 != Rows B=2)
        let a = DenseMatrix::new(2, 3, vec![1.0; 6]);
        let b = DenseMatrix::new(2, 2, vec![1.0; 4]);
        
        // Ahora capturamos el error en lugar de esperar un panic
        match &a * &b {
            Err(LinearAlgebraError::DimensionMismatch { expected, found, .. }) => {
                assert_eq!(expected, 3);
                assert_eq!(found, 2);
            },
            Ok(_) => panic!("Debería haber fallado por dimensión"),
            _ => panic!("Tipo de error incorrecto"),
        }
    }
}