use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError;

impl<T> DenseMatrix<T>
where
    T: Scalar,
{
    /// Calcula la inversa de la matriz utilizando el método de Gauss-Jordan.
    /// Retorna error si la matriz no es cuadrada o si es singular (no tiene inversa).
    pub fn inverse(&self) -> Result<DenseMatrix<T>, LinearAlgebraError> {
        // 1. Validación: Debe ser Cuadrada
        if self.rows != self.cols {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Inverse".to_string(),
                expected: self.rows, // Esperábamos que cols == rows
                found: self.cols,
            });
        }

        let n = self.rows;

        // 2. Construcción de la Matriz Aumentada [A | I]
        // Tamaño: n filas, 2n columnas
        let mut augmented_data = Vec::with_capacity(n * (2 * n));

        for i in 0..n {
            // Parte izquierda (Copia de A)
            for j in 0..n {
                augmented_data.push(self.get(i, j));
            }
            // Parte derecha (Identidad)
            for j in 0..n {
                if i == j {
                    augmented_data.push(T::one());
                } else {
                    augmented_data.push(T::zero());
                }
            }
        }

        let mut augmented = DenseMatrix::new(n, 2 * n, augmented_data);

        // 3. Aplicar Gauss-Jordan
        augmented.rref()?;

        // 4. Extracción y Verificación de Singularidad
        // Si A es invertible, la parte izquierda DEBE ser la matriz identidad.
        // Verificamos que la diagonal principal sea 1 (o algo normalizado a 1).
        // Si encontramos un 0 en la diagonal después de RREF, es singular.
        
        let mut inverse_data = Vec::with_capacity(n * n);

        for i in 0..n {
            // Verificación rápida de singularidad:
            // En RREF, si A es invertible, A[i][i] debe ser 1.
            let diag_val = augmented.get(i, i);
            if diag_val.is_zero() {
                // Nota: Podríamos agregar un error específico "SingularMatrix" en LinearAlgebraError
                return Err(LinearAlgebraError::DimensionMismatch { 
                    operation: "Inverse (Singular Matrix check)".to_string(),
                    expected: 1, 
                    found: 0 
                });
            }

            // Extraer parte derecha (columnas de n a 2n)
            for j in 0..n {
                inverse_data.push(augmented.get(i, n + j));
            }
        }

        Ok(DenseMatrix::new(n, n, inverse_data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
    use crate::matrix;
    use crate::symbolics::ast::{var, Expr};
    // Asumiendo que matrix! está disponible globalmente por #[macro_export]
    // o importado correctamente.

    #[test]
    fn test_inverse_numeric_2x2() {
        // Matriz A:
        // [ 4  7 ]
        // [ 2  6 ]
        let a = matrix![
            4.0, 7.0;
            2.0, 6.0
        ];

        let inv = a.inverse().expect("La matriz es invertible");
        
        // Inversa Esperada Teórica:
        // [ 0.6  -0.7 ]
        // [ -0.2  0.4 ]
        let expected = matrix![
             0.6, -0.7;
            -0.2,  0.4
        ];
        
        // CORRECCIÓN ARQUITECTÓNICA:
        // Usamos is_approx porque IEEE 754 no garantiza igualdad exacta (0.600...01 != 0.6)
        // Si inv.is_approx no existe, asegúrate de haberlo agregado a DenseMatrix
        // como discutimos en el paso anterior.
        if !inv.is_approx(&expected) {
            panic!(
                "Fallo de precisión numérica.\nObtenida: {:?}\nEsperada: {:?}", 
                inv, expected
            );
        }
    }

    #[test]
    fn test_inverse_identity() {
        let i = matrix![
            1.0, 0.0;
            0.0, 1.0
        ];
        
        let inv = i.inverse().expect("Identidad es invertible");
        
        // La inversa de la identidad es ella misma.
        // Usamos is_approx por seguridad, aunque con 1.0 y 0.0 suele ser exacto.
        assert!(inv.is_approx(&i), "La inversa de la identidad debe ser la identidad");
    }

    #[test]
    fn test_inverse_symbolic_diagonal() {
        // Matriz Simbólica Diagonal:
        // [ x  0 ]
        // [ 0  y ]
        let x = var("x");
        let y = var("y");

        let a = matrix![
            x.clone(), Expr::from(0.0);
            Expr::from(0.0), y.clone()
        ];

        let inv = a.inverse().expect("Inversión simbólica falló");

        // Verificamos (0,0) -> 1/x
        // Aquí SÍ usamos assert_eq! porque para símbolos la igualdad debe ser estructural y exacta.
        // Scalar::is_approx para Expr llama a ==, así que es lo mismo.
        let cell_00 = inv.get(0, 0).simplify();
        match cell_00 {
            Expr::Div(lhs, rhs) => {
                assert_eq!(*lhs, Expr::Const(1.0));
                assert_eq!(*rhs, x);
            },
            _ => panic!("Esperaba estructura 1/x, obtuve {:?}", cell_00),
        }

        // Verificamos (1,1) -> 1/y
        let cell_11 = inv.get(1, 1).simplify();
        match cell_11 {
            Expr::Div(lhs, rhs) => {
                assert_eq!(*lhs, Expr::Const(1.0));
                assert_eq!(*rhs, y);
            },
            _ => panic!("Esperaba estructura 1/y, obtuve {:?}", cell_11),
        }
    }
}