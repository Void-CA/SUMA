use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError;

impl<T> DenseMatrix<T>
where
    T: Scalar,
{
    pub fn determinant(&self) -> Result<T, LinearAlgebraError> {
        if self.rows != self.cols {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Determinant".to_string(),
                expected: self.rows,
                found: self.cols,
            });
        }

        let n = self.rows;
        // Trabajamos sobre una copia para no modificar la original
        let mut mat = self.clone(); 
        let mut swaps = 0;

        // Algoritmo de eliminación Gaussiana (Sin llevar a RREF completo, solo Triangular Superior)
        for i in 0..n {
            // 1. Pivoteo
            let mut pivot_row = i;
            while pivot_row < n && mat.get(pivot_row, i).is_zero() {
                pivot_row += 1;
            }

            if pivot_row == n {
                // Si no encontramos pivote en esta columna, el determinante es 0
                return Ok(T::zero());
            }

            if pivot_row != i {
                mat.swap_rows(i, pivot_row)?;
                swaps += 1;
            }

            // 2. Eliminación (Hacer ceros DEBAJO del pivote solamente)
            for j in (i + 1)..n {
                let pivot = mat.get(i, i);
                let target = mat.get(j, i);

                if !target.is_zero() {
                    // Queremos hacer target cero.
                    // R_j = R_j - (target/pivot) * R_i
                    // Cuidado: add_scaled_row hace: R_tgt + (scalar * R_src)
                    // Entonces scalar = -(target/pivot)
                    
                    // Nota: Aquí asumimos que Scalar soporta división.
                    // Para evitar divisiones complejas en simbólico, a veces se usa Fraction-Free Gaussian,
                    // pero con tu sistema de Expr actual, esto generará expresiones anidadas que simplify() manejará.
                    
                    let scalar = -(target / pivot);
                    mat.add_scaled_row(j, i, scalar)?;
                }
            }
        }

        // 3. Producto de la Diagonal
        let mut det = T::one();
        for i in 0..n {
            det = det * mat.get(i, i);
        }

        // 4. Ajuste de Signo por Swaps
        // Si swaps es impar, multiplicamos por -1
        if swaps % 2 != 0 {
            det = -det;
        }

        Ok(det)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
    use crate::matrix;
    use crate::symbolics::ast::{var, Expr};
    // Asumiendo que matrix! está disponible
    
    #[test]
    fn test_determinant_numeric_2x2() {
        // [ 1  2 ]
        // [ 3  4 ]
        // det = 1*4 - 2*3 = 4 - 6 = -2
        let m = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];
        
        let det = m.determinant().expect("Cálculo exitoso");
        
        // Usamos is_approx del trait Scalar (que implementamos para f64)
        assert!(det.is_approx(&-2.0), "Det obtenido: {}, Esperado: -2.0", det);
    }

    #[test]
    fn test_determinant_singular() {
        // Matriz con filas linealmente dependientes (R2 = 2*R1)
        // [ 1  2 ]
        // [ 2  4 ] 
        // det = 4 - 4 = 0
        let m = matrix![
            1.0, 2.0;
            2.0, 4.0
        ];
        
        let det = m.determinant().expect("Cálculo exitoso");
        assert!(det.is_approx(&0.0));
    }

    #[test]
    fn test_determinant_triangular() {
        // El determinante de una triangular es el producto de su diagonal
        // [ 2  5  1 ]
        // [ 0  3  2 ]
        // [ 0  0  4 ]
        // det = 2 * 3 * 4 = 24
        let m = matrix![
            2.0, 5.0, 1.0;
            0.0, 3.0, 2.0;
            0.0, 0.0, 4.0
        ];
        
        let det = m.determinant().unwrap();
        assert!(det.is_approx(&24.0));
    }

    #[test]
    fn test_determinant_symbolic_identity() {
        // Probamos que la identidad simbólica dé 1
        // [ 1  0 ]
        // [ 0  1 ]
        let m = matrix![
            Expr::from(1.0), Expr::from(0.0);
            Expr::from(0.0), Expr::from(1.0)
        ];

        let det = m.determinant().unwrap().simplify();
        assert_eq!(det, Expr::Const(1.0));
    }

    #[test]
    fn test_determinant_symbolic_simple() {
        // Matriz diagonal simbólica
        // [ x  0 ]
        // [ 0  y ]
        // det = x * y
        let x = var("x");
        let y = var("y");
        
        let m = matrix![
            x.clone(), Expr::from(0.0);
            Expr::from(0.0), y.clone()
        ];

        let det = m.determinant().unwrap().simplify();
        
        // Verificamos estructura
        match det {
            Expr::Mul(lhs, rhs) => {
                // Puede ser x*y o y*x dependiendo del orden de ejecución
                let case1 = *lhs == x && *rhs == y;
                let case2 = *lhs == y && *rhs == x;
                assert!(case1 || case2, "Esperaba x*y");
            },
            _ => panic!("Estructura incorrecta para det diagonal: {:?}", det),
        }
    }
}