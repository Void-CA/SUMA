use serde::Serialize;
use suma_core::symbolics::ast::Expr; // Reutilizamos la expresión simbólica

// --- 1. El Bloque Contenedor (Lo que faltaba) ---
// Este es el struct que el Parser devuelve envuelto en Box<...>
#[derive(Debug, Clone, Serialize)]
pub enum OptimizationBlock {
    Definition(OptimizationModel),
    Query(OptimizationQuery),
}

// --- 2. El Modelo de Datos del Dominio ---

#[derive(Debug, Clone, Serialize)]
pub struct OptimizationModel {
    pub name: String,
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

// --- QUERY (Ejecución) ---
#[derive(Debug, Clone, Serialize)]
pub struct OptimizationQuery {
    pub target_id: String,
    pub requests: Vec<OptimizationRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub enum OptimizationRequest {
    Solve,
    ShadowPrices,
    CheckFeasibility,
}
