use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;

#[derive(Debug, Clone)]
pub struct SimplexTableau {
    pub matrix: DenseMatrix<f64>, 
    /// Índices de las variables que están en la base (asociadas a cada fila)
    pub basic_vars: Vec<usize>,    
    /// Índices de las variables fuera de la base (opcional, útil para debug)
    pub non_basic_vars: Vec<usize>,
}

impl SimplexTableau {
    /// Ejecuta la operación de Pivoteo (Gaussian Pivot).
    /// 1. Divide la fila pivote por el elemento pivote (para hacerlo 1).
    /// 2. Resta múltiplos de la fila pivote a todas las demás filas (para hacerlas 0).
    /// 3. Actualiza el registro de variables básicas.
    pub fn pivot(&mut self, pivot_row: usize, pivot_col: usize) {
        let rows = self.matrix.rows;
        let cols = self.matrix.cols;

        // 1. Obtener el valor del elemento pivote
        let pivot_val = self.matrix.get(pivot_row, pivot_col);

        // Seguridad numérica básica: si el pivote es casi cero, explotaría
        if pivot_val.abs() < 1e-12 {
            // En un sistema real, aquí intentaríamos re-condicionar la matriz
            panic!("Error numérico: Pivote cercano a cero en ({}, {})", pivot_row, pivot_col);
        }

        // 2. Normalizar la fila pivote (hacer que el pivote sea 1.0)
        // R_pivot = R_pivot / pivot_val
        for j in 0..cols {
            let val = self.matrix.get(pivot_row, j);
            self.matrix.set(pivot_row, j, val / pivot_val);
        }

        // 3. Hacer ceros en el resto de la columna (Gaussian Elimination)
        // R_i = R_i - (factor * R_pivot)
        for i in 0..rows {
            if i != pivot_row {
                let factor = self.matrix.get(i, pivot_col);
                
                // Si el factor ya es 0, no necesitamos hacer nada (optimización de dispersión)
                if factor.abs() > 1e-12 {
                    for j in 0..cols {
                        let row_val_pivot = self.matrix.get(pivot_row, j); // Valor normalizado
                        let current_val = self.matrix.get(i, j);
                        self.matrix.set(i, j, current_val - (factor * row_val_pivot));
                    }
                }
            }
        }

        // 4. Actualizar la base
        // La variable que estaba en pivot_row sale, entra la variable pivot_col
        self.basic_vars[pivot_row] = pivot_col;
        
        // Nota: Actualizar non_basic_vars es opcional para el algoritmo primal básico,
        // pero mantenemos la estructura limpia si decidimos usarla.
        if let Some(pos) = self.non_basic_vars.iter().position(|&x| x == pivot_col) {
            self.non_basic_vars.remove(pos);
            // Aquí deberíamos insertar la variable que salió, pero necesitamos saber cuál era.
            // Para el cálculo puro, basta con actualizar basic_vars.
        }
    }
}