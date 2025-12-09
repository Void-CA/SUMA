// src/ast/mod.rs
use serde::Serialize;
// Importamos los modelos de los dominios
use crate::domains::optimization::OptimizationModel;
use crate::domains::boolean_algebra::BooleanModel;

#[derive(Debug, Serialize, Clone)]
pub enum CodexResult {
    Optimization(OptimizationModel),
    Boolean(BooleanModel),
}