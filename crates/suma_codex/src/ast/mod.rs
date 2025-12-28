use serde::Serialize;

// Importamos los modelos de los dominios
use crate::domains::optimization::OptimizationModel;
use crate::domains::boolean_algebra::BooleanModel;
// Importamos el nuevo modelo serializable
use crate::domains::linear_algebra::ast::LinearAlgebraModel;

#[derive(Debug, Serialize, Clone)]
pub enum CodexResult {
    // Resultado de Optimización (Simplex/Interior Point)
    Optimization(OptimizationModel),

    // Resultado de Lógica (SAT/Evaluation)
    Boolean(BooleanModel),

    // Resultado de Matrices (Gauss-Jordan/Systems)
    LinearAlgebra(LinearAlgebraModel),
}