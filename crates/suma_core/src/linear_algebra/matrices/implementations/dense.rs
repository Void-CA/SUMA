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

    /// Attempts to create a new DenseMatrix, returning an error if the data length doesn't match.
    pub fn try_new(rows: usize, cols: usize, data: Vec<T>) -> Result<Self, crate::linear_algebra::error::LinearAlgebraError> {
        if data.len() != rows * cols {
            return Err(crate::linear_algebra::error::LinearAlgebraError::DimensionMismatch {
                operation: format!("Expected {} elements, got {}", rows * cols, data.len()),
                expected: rows * cols,
                found: data.len(),
            });
        }
        Ok(Self { data, rows, cols })
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(self.data[row * self.cols + col].clone())
    }

    pub fn get_ref(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(&self.data[row * self.cols + col])
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![T::zero(); rows * cols]; 
        Self { data, rows, cols }
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), crate::linear_algebra::error::LinearAlgebraError> {
        if row >= self.rows || col >= self.cols {
            return Err(crate::linear_algebra::error::LinearAlgebraError::IndexOutOfBounds {
                context: format!("Index ({}, {}) for {}x{} matrix", row, col, self.rows, self.cols),
                index: if row >= self.rows { row } else { col },
                max: if row >= self.rows { self.rows } else { self.cols },
            });
        }
        self.data[row * self.cols + col] = value;
        Ok(())
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

        for i in 0..self.rows {
            let mut row_strs = Vec::with_capacity(self.cols);
            for j in 0..self.cols {
                let val = self.get(i, j).unwrap();
                
                // Use precision formatting only when explicitly requested (e.g. {:.2})
                let s = match f.precision() {
                    Some(prec) => format!("{:.1$}", val, prec),
                    None => format!("{}", val),
                };
                
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