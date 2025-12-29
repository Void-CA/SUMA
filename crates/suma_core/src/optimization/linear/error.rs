use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Estado final del solucionador
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationStatus {
    Optimal,
    // Suboptimal, // Para cuando implementemos límites de tiempo/iteraciones
}

/// La estructura de éxito que devuelve el solver
#[derive(Debug, Clone)]
pub struct Solution {
    pub status: OptimizationStatus,
    pub objective_value: f64,
    pub variables: HashMap<String, f64>, // Mapa: "NombreVariable" -> Valor

    pub shadow_prices: HashMap<String, f64>, // Valores duales para cada restricción
}

/// Los errores específicos de Programación Lineal
#[derive(Debug, Clone)]
pub enum LinearOptimizationError {
    /// El conjunto de restricciones es contradictorio (Región factible vacía)
    Infeasible,
    
    /// La función objetivo tiende a +/- infinito en la región factible
    Unbounded,
    
    /// Se alcanzó el límite de iteraciones (posible ciclo o problema numérico)
    MaxIterationsReached,
    
    /// Error numérico (ej. matriz singular durante pivoteo)
    NumericalError(String),
    
    /// Error de validación previo al cálculo (ej. dimensiones incorrectas)
    ValidationError(String),

    /// Error al transformar estructuras (ej. NonLinear en un solver lineal)
    NonLinearExpression(String),
}

// Implementación de Display para logs claros
impl fmt::Display for LinearOptimizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinearOptimizationError::Infeasible => write!(f, "El problema no tiene solución factible."),
            LinearOptimizationError::Unbounded => write!(f, "El problema es no acotado (solución infinita)."),
            LinearOptimizationError::MaxIterationsReached => write!(f, "Límite de iteraciones alcanzado."),
            LinearOptimizationError::NumericalError(msg) => write!(f, "Error numérico: {}", msg),
            LinearOptimizationError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            LinearOptimizationError::NonLinearExpression(msg) => write!(f, "Expresión no lineal: {}", msg),
        }
    }
}

/// Alias conveniente para el retorno de funciones del solver
pub type OptimizationResult = Result<Solution, LinearOptimizationError>;