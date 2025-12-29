use thiserror::Error;
use super::linear::error::LinearOptimizationError;

#[derive(Debug, Error)]
pub enum OptimizationError {
    /// Errores provenientes del motor de Programación Lineal
    #[error(transparent)]
    Linear(#[from] LinearOptimizationError),

    // Futuro: Errores de Programación Entera
    // #[error(transparent)]
    // Integer(#[from] IntegerOptimizationError),

    // Futuro: Errores no lineales
    // #[error(transparent)]
    // NonLinear(#[from] NonLinearError),
    
    // Errores genéricos del módulo de optimización (si fueran necesarios)
    #[error("Configuración de optimizador inválida: {0}")]
    ConfigError(String),
}