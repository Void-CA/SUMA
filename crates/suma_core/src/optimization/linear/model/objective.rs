use std::fmt;
use super::LinearExpression;

/// Define la dirección de la optimización.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationDirection {
    Maximize,
    Minimize,
}

/// Representa la Función Objetivo del problema (Z).
/// Ejemplo: Max Z = 3x + 2y
#[derive(Debug, Clone, PartialEq)]
pub struct Objective {
    pub direction: OptimizationDirection,
    pub expression: LinearExpression,
}

impl Objective {
    /// Constructor genérico
    pub fn new(direction: OptimizationDirection, expression: LinearExpression) -> Self {
        Self { direction, expression }
    }

    /// Constructor rápido para Maximización
    pub fn maximize(expression: LinearExpression) -> Self {
        Self::new(OptimizationDirection::Maximize, expression)
    }

    /// Constructor rápido para Minimización
    pub fn minimize(expression: LinearExpression) -> Self {
        Self::new(OptimizationDirection::Minimize, expression)
    }
}

// Implementación de Display para mostrarlo bonito en logs o consola
// Ejemplo output: "Maximize: 3*x + 2*y + 0"
impl fmt::Display for Objective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dir_str = match self.direction {
            OptimizationDirection::Maximize => "Maximize",
            OptimizationDirection::Minimize => "Minimize",
        };
        write!(f, "{}: {}", dir_str, self.expression)
    }
}