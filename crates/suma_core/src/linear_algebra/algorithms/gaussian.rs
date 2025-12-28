use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError;

impl<T> DenseMatrix<T>
where
    T: Scalar, // Recuerda: T tiene Zero, One, Add, Sub, Mul, Div...
{
    /// Transforma la matriz a su Forma Escalonada Reducida por Filas (RREF)
    /// utilizando el algoritmo de Gauss-Jordan.
    pub fn rref(&mut self) -> Result<(), LinearAlgebraError> {
        let mut pivot_row = 0;

        // Iteramos sobre las columnas (j)
        for j in 0..self.cols {
            if pivot_row >= self.rows {
                break;
            }

            // 1. ESTRATEGIA DE PIVOTEO
            // Buscamos una fila (desde pivot_row hacia abajo) que tenga un valor no-cero en la columna j.
            let mut pivot_found = false;
            for k in pivot_row..self.rows {
                if !self.get_ref(k, j).is_zero() {
                    // Encontramos un pivote no nulo. Lo traemos a la posición actual.
                    self.swap_rows(pivot_row, k)?;
                    pivot_found = true;
                    break;
                }
            }

            if !pivot_found {
                // Si toda la columna j tiene ceros (desde pivot_row), pasamos a la siguiente columna.
                continue;
            }

            // 2. NORMALIZACIÓN
            // Hacemos que el pivote sea 1 (Dividiendo toda la fila por el valor del pivote)
            // R_pivot = R_pivot / pivot_val
            let pivot_val = self.get(pivot_row, j); // Clonamos el valor
            
            // Optimización: Si ya es 1, no hacemos nada.
            // Nota: Esto requiere que Scalar pueda compararse con 1, lo asumimos por eficiencia genérica dividiendo.
            // Para evitar división por cero (aunque chequeamos is_zero arriba), manejamos lógica:
            
            // Invertimos el valor: inv = 1 / pivot
            let inv_pivot = T::one() / pivot_val; 
            self.scale_row(pivot_row, inv_pivot)?;

            // 3. ELIMINACIÓN (Hacer ceros arriba y abajo)
            for k in 0..self.rows {
                if k != pivot_row {
                    let factor = self.get(k, j); // El valor que queremos eliminar
                    if !factor.is_zero() {
                        // R_k = R_k - (factor * R_pivot)
                        // Nuestra función es add_scaled_row(target, source, scalar)
                        // R_k = R_k + (-factor * R_pivot)
                        self.add_scaled_row(k, pivot_row, -factor)?;
                    }
                }
            }

            // 4. MANTENIMIENTO SIMBÓLICO (CRÍTICO)
            // Aquí es donde deberíamos llamar a simplify() en cada celda si T es Expr.
            // Como T es genérico, confiamos en que las operaciones de fila ya manejen cierta lógica,
            // o que el usuario llame a simplify() al final.
            // (Para una implementación perfecta, Scalar podría tener un método .simplify_in_place())

            pivot_row += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
    use crate::matrix;
    use crate::symbolics::ast::{var, Expr};
    // Importamos el macro. Dependiendo de dónde lo definiste, 
    // podría ser `use crate::matrix;` o estar disponible automáticamente por #[macro_export]
    
    #[test]
    fn test_rref_simple_system_numeric() {
        // Sistema 2x2:
        // 2x + y = 5
        // x + 3y = 5
        
        // Matriz Aumentada [ A | b ]
        // Definición limpia gracias a tu macro:
        let mut m = matrix![
            2.0, 1.0, 5.0;
            1.0, 3.0, 5.0
        ];

        m.rref().expect("RREF debería funcionar");

        // Resultado esperado: Identidad | Solución (x=2, y=1)
        let expected = matrix![
            1.0, 0.0, 2.0;
            0.0, 1.0, 1.0
        ];

        assert_eq!(m, expected);
    }

    #[test]
    fn test_rref_infinite_solutions_numeric() {
        // Sistema Singular (Filas dependientes)
        let mut m = matrix![
            1.0, 2.0, 3.0;
            2.0, 4.0, 6.0
        ];

        m.rref().expect("No debe fallar en matrices singulares");

        // Fila 2 debe anularse
        let expected = matrix![
            1.0, 2.0, 3.0;
            0.0, 0.0, 0.0
        ];

        assert_eq!(m, expected);
    }

    #[test]
    fn test_rref_symbolic_structure() {
        // [ 1  x ]
        // [ 0  1 ]
        // R0 = R0 - x*R1
        
        // Nota: Para mezclar literales y Expr, envolvemos los literales.
        // O mejor aún, usamos Expr::from() implícito si tu macro lo soportara,
        // pero por seguridad en Rust explícito:
        let x = var("x");
        
        let mut m = matrix![
            Expr::from(1.0), x.clone();
            Expr::from(0.0), Expr::from(1.0)
        ];

        m.rref().expect("RREF simbólico falló");

        // Verificamos la eliminación del elemento (0, 1)
        let cell_target = m.get(0, 1);
        
        // Validación rigurosa: (x - x*1) -> 0
        assert_eq!(cell_target.simplify(), Expr::Const(0.0));
    }
}