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