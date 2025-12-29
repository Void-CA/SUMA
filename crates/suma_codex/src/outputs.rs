use suma_core::linear_algebra::DenseMatrix;
// Podrías agregar aquí: use crate::domains::optimization::OptimizationResult; etc.

/// Enum universal para cualquier salida producida por el Codex.
/// El CLI usará esto para decidir si pintar un número, una tabla o un error.
#[derive(Debug, Clone)]
pub enum CodexOutput {
    // --- Salidas de Álgebra Lineal ---
    LinAlgScalar(f64),
    LinAlgVector(DenseMatrix<f64>), // Usamos DenseMatrix porque un vector es una matriz Nx1
    LinAlgMatrix(DenseMatrix<f64>),
    
    // --- Salidas Genéricas ---
    Message(String),      // Mensajes informativos simples
    Error(String),        // Errores de runtime controlados
    
    // A futuro agregarás aquí:
    // OptimizationResult(...),
    // BooleanTable(...),
}