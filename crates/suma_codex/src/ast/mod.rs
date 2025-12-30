use serde::Serialize;

// Importamos los modelos de los dominios
use crate::domains::boolean_algebra::BooleanModel;
// Importamos el nuevo modelo serializable
use crate::domains::linear_algebra::ast::LinearAlgebraBlock;
use crate::domains::optimization::ast::OptimizationBlock;
use crate::domains::queries::ast::QueryBlock;

#[derive(Debug, Serialize, Clone)]
pub enum CodexResult {
    // Resultado de Optimización (Simplex/Interior Point)
    Optimization(OptimizationBlock),

    // Resultado de Lógica (SAT/Evaluation)
    Boolean(BooleanModel),

    // Resultado de Matrices (Gauss-Jordan/Systems)
    LinearAlgebra(LinearAlgebraBlock),

    Query(QueryBlock),
}