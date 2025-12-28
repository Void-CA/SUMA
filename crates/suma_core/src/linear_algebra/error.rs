use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum LinearAlgebraError {
    DimensionMismatch {
        operation: String,
        expected: usize,
        found: usize,
    },
    // NUEVO: Para proteger row_ops
    IndexOutOfBounds {
        context: String, // "Fila", "Columna"
        index: usize,
        max: usize,      // El límite que se violó
    },
}

impl fmt::Display for LinearAlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinearAlgebraError::DimensionMismatch { operation, expected, found } => {
                write!(f, "[Error {}]: Se esperaba {}, se encontró {}.", operation, expected, found)
            }
            // Mensaje claro para el usuario
            LinearAlgebraError::IndexOutOfBounds { context, index, max } => {
                write!(f, "Error de Índice: Intento de acceder a {} {}, pero el máximo permitido es {}.", context, index, max - 1)
            }
        }
    }
}

// 2. Implementación de std::error::Error (Para integración con anyhow/thiserror)
impl Error for LinearAlgebraError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}