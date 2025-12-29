use std::fmt;

// src/linear_algebra/matrices/implementations/dense.rs
use crate::linear_algebra::traits::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct DenseMatrix<T>
where
    T: Scalar 
{
    pub data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> DenseMatrix<T>
where
    T: Scalar
{
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length mismatch");
        Self { data, rows, cols }
    }

    // Cambio importante: Ahora devuelve T clonándolo, porque T podría no ser Copy (Expr)
    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col].clone() 
    }

    // Opcional: Una versión más eficiente que devuelve referencia
    pub fn get_ref(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        // T::zero() viene de nuestro trait Zero
        // vec! con clone requiere que T sea Clone, lo cual Scalar garantiza.
        let data = vec![T::zero(); rows * cols]; 
        Self { data, rows, cols }
    }
    
    pub fn is_approx(&self, other: &DenseMatrix<T>) -> bool {
        // 1. Si las dimensiones son distintas, imposible que sean iguales
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }

        // 2. Comparamos elemento a elemento
        // Usamos iteradores (zip) para recorrer ambos vectores simultáneamente.
        // .all() devuelve true solo si TODOS los pares cumplen la condición.
        self.data.iter()
            .zip(other.data.iter())
            .all(|(val_self, val_other)| val_self.is_approx(val_other))
    }
}

impl<T> fmt::Display for DenseMatrix<T>
where
    T: Scalar
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Pre-cálculo: Convertimos a String para medir longitudes.
        // Esto es necesario para la alineación perfecta.
        let mut string_rows: Vec<Vec<String>> = Vec::with_capacity(self.rows);
        let mut max_col_width = 0;

        // Detectamos si el usuario pidió precisión (ej: {:.2})
        let precision = f.precision().unwrap_or(4); // Default 4 decimales si es float

        for i in 0..self.rows {
            let mut row_strs = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                let val = self.get(i, j);
                
                // Truco: Intentamos formatear con precisión si es flotante.
                // Como T es genérico, format!("{:.p$}", val) funciona si T soporta precisión.
                // Si T es entero, ignorará la precisión o usaremos el default.
                // Para simplificar en contexto genérico, usamos format! estándar:
                let s = format!("{:.1$}", val, precision); 
                
                if s.len() > max_col_width {
                    max_col_width = s.len();
                }
                row_strs.push(s);
            }
            string_rows.push(row_strs);
        }

        // 2. Renderizado: Imprimimos con márgenes y bordes
        for (i, row_strs) in string_rows.iter().enumerate() {
            // Decoración de bordes (estilo académico)
            let left_border = if self.rows == 1 { "[" } 
                              else if i == 0 { "┌" } 
                              else if i == self.rows - 1 { "└" } 
                              else { "│" };

            let right_border = if self.rows == 1 { "]" } 
                               else if i == 0 { "┐" } 
                               else if i == self.rows - 1 { "┘" } 
                               else { "│" };

            write!(f, "{}", left_border)?;

            for (j, s) in row_strs.iter().enumerate() {
                // Padding: Alineamos a la derecha para números
                write!(f, " {:>width$}", s, width = max_col_width)?;
                
                // Espacio entre columnas (menos en la última)
                if j < self.cols - 1 {
                    write!(f, "  ")?; 
                }
            }

            writeln!(f, " {}", right_border)?;
        }

        Ok(())
    }
}