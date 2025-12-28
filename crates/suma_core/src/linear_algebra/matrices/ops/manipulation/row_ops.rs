use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError;

impl<T> DenseMatrix<T>
where
    T: Scalar,
{
    /// Validador auxiliar privado
    fn check_row_bounds(&self, row: usize) -> Result<(), LinearAlgebraError> {
        if row >= self.rows {
            return Err(LinearAlgebraError::IndexOutOfBounds {
                context: "Fila".to_string(),
                index: row,
                max: self.rows,
            });
        }
        Ok(())
    }

    /// Operación 1: Intercambio de Filas (Swap)
    /// R_i <-> R_j
    pub fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), LinearAlgebraError> {
        self.check_row_bounds(row1)?;
        self.check_row_bounds(row2)?;

        if row1 == row2 {
            return Ok(()); // No hacer nada es una operación válida
        }

        // Iteramos columna por columna intercambiando los elementos
        // Nota: Al ser un Vec plano, calculamos índices manualmente.
        for k in 0..self.cols {
            let idx1 = row1 * self.cols + k;
            let idx2 = row2 * self.cols + k;
            self.data.swap(idx1, idx2);
        }

        Ok(())
    }

    /// Operación 2: Escalar Fila (Scale)
    /// R_i -> k * R_i
    pub fn scale_row(&mut self, row: usize, scalar: T) -> Result<(), LinearAlgebraError> {
        self.check_row_bounds(row)?;

        // Optimización: Si el escalar es 1, no hacemos nada (identidad)
        // (Podríamos chequear si es 0 para advertir, pero matemáticamente es válido anular una fila)
        
        for k in 0..self.cols {
            let idx = row * self.cols + k;
            // self.data[idx] = self.data[idx] * scalar
            // Usamos clone() porque T es Scalar (no necesariamente Copy)
            let val = self.data[idx].clone();
            self.data[idx] = val * scalar.clone();
        }

        Ok(())
    }

    /// Operación 3: Sumar Fila Escalada (Add Scaled)
    /// R_target -> R_target + (scalar * R_source)
    /// Esta es la operación "workhorse" de la eliminación gaussiana.
    pub fn add_scaled_row(&mut self, target_row: usize, source_row: usize, scalar: T) -> Result<(), LinearAlgebraError> {
        self.check_row_bounds(target_row)?;
        self.check_row_bounds(source_row)?;

        for k in 0..self.cols {
            let src_idx = source_row * self.cols + k;
            let tgt_idx = target_row * self.cols + k;

            // R_tgt = R_tgt + (k * R_src)
            let src_val = self.data[src_idx].clone();
            let tgt_val = self.data[tgt_idx].clone();
            
            self.data[tgt_idx] = tgt_val + (scalar.clone() * src_val);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbolics::ast::{var, Expr};

    #[test]
    fn test_symbolic_row_operation() {
        // Matriz Sistema:
        // [ 1   x ]  <- Fila 0
        // [ 0   1 ]  <- Fila 1
        // Queremos hacer R0 = R0 - x*R1  (Para eliminar la 'x' y diagonalizar)
        
        // Setup
        let x = var("x");
        let mut matrix = DenseMatrix::new(2, 2, vec![
            Expr::from(1.0), x.clone(),
            Expr::from(0.0), Expr::from(1.0)
        ]);

        // Operación: R0 = R0 + (-x * R1)
        let scalar = -x; // Neg(Var("x"))
        matrix.add_scaled_row(0, 1, scalar).expect("Operación válida");

        // Verificación
        // La celda (0, 1) era 'x'.
        // Operación: x + (-x * 1) 
        // Resultado esperado (sin simplificar): Add(x, Mul(-x, 1))
        
        let target_cell = matrix.get(0, 1);
        println!("Celda resultante (debe reducirse a 0): {:?}", target_cell);

        // Si tuviéramos simplify(), esto daría Const(0.0).
        // Por ahora, validamos que la estructura cambió.
        assert_ne!(target_cell, var("x")); 
    }

    #[test]
    fn test_index_out_of_bounds() {
        let mut m: DenseMatrix<Expr> = DenseMatrix::zeros(2, 2);
        // Intentar acceder a fila 5
        let result = m.swap_rows(0, 5);
        
        match result {
            Err(LinearAlgebraError::IndexOutOfBounds { index, max, .. }) => {
                assert_eq!(index, 5);
                assert_eq!(max, 2);
            },
            _ => panic!("Debería dar error de índice"),
        }
    }
}