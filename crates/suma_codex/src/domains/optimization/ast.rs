use serde::Serialize;
use suma_core::symbolics::ast::Expr; // Reutilizamos la expresión simbólica

// --- 1. El Bloque Contenedor (Lo que faltaba) ---
// Este es el struct que el Parser devuelve envuelto en Box<...>
#[derive(Debug, Clone, Serialize)]
pub struct OptimizationBlock {
    pub model: OptimizationModel,
}

// --- 2. El Modelo de Datos del Dominio ---

#[derive(Debug, Clone, Serialize)]
pub struct OptimizationModel {
    pub id: String,
    pub direction: OptimizationDirection,
    pub objective: Expr,
    pub constraints: Vec<ConstraintModel>,
}

#[derive(Debug, Clone, Serialize)]
pub enum OptimizationDirection {
    Maximize,
    Minimize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstraintModel {
    pub left: Expr,
    pub relation: String, // "<=", ">=", "="
    pub right: Expr,
}

// Implementación opcional para facilitar la creación en tests
impl OptimizationBlock {
    pub fn new(model: OptimizationModel) -> Self {
        Self { model }
    }
}