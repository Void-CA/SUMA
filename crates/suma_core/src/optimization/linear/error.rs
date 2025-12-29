use std::collections::HashMap;
use thiserror::Error; // Necesitas agregar 'thiserror' a las dependencias si no está visible aquí

// Mantenemos OptimizationStatus y Solution igual...
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationStatus {
    Optimal,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub status: OptimizationStatus,
    pub objective_value: f64,
    pub variables: HashMap<String, f64>,
    pub shadow_prices: HashMap<String, f64>,
}

/// Errores específicos de Programación Lineal usando `thiserror`
#[derive(Debug, Clone, Error)]
pub enum LinearOptimizationError {
    #[error("El problema no tiene solución factible (Infeasible).")]
    Infeasible,
    
    #[error("El problema es no acotado (Unbounded).")]
    Unbounded,
    
    #[error("Límite de iteraciones alcanzado.")]
    MaxIterationsReached,
    
    #[error("Error numérico: {0}")]
    NumericalError(String),
    
    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Expresión no lineal detectada: {0}")]
    NonLinearExpression(String),
}

pub type OptimizationResult = Result<Solution, LinearOptimizationError>;