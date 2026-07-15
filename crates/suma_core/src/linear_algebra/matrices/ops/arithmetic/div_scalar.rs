use std::ops::Div;
use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;

// Caso: Matriz * Escalar (A * k)
impl<T> Div<T> for DenseMatrix<T>
where
    T: Scalar,
{
    type Output = DenseMatrix<T>;

    fn div(self, scalar: T) -> Self::Output {
        let new_data: Vec<T> = self.data
            .into_iter() // Consumimos la data original (o usa iter() y clone si prefieres &self)
            .map(|val| val / scalar.clone())
            .collect();

        DenseMatrix::new(self.rows, self.cols, new_data)
    }
}

// Caso: Referencia a Matriz * Escalar (&A * k)
impl<'a, T> Div<T> for &'a DenseMatrix<T>
where
    T: Scalar,
{
    type Output = DenseMatrix<T>;

    fn div(self, scalar: T) -> Self::Output {
        let new_data: Vec<T> = self.data
            .iter()
            .map(|val| val.clone() / scalar.clone())
            .collect();

        DenseMatrix::new(self.rows, self.cols, new_data)
    }
}

#[cfg(test)]
mod tests {
    use crate::linear_algebra::DenseMatrix;

    #[test]
    fn test_div_scalar() {
        let a = DenseMatrix::new(2, 2, vec![2.0, 4.0, 6.0, 8.0]);
        let result = &a / 2.0;
        assert_eq!(result, DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_div_scalar_one() {
        let a = DenseMatrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = &a / 1.0;
        assert_eq!(result, a);
    }
}