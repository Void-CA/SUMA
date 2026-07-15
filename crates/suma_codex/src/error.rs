use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodexError {
    #[error("Parse error in domain '{domain}': {message}")]
    ParseError { domain: String, message: String },

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Unknown domain: {0}")]
    UnknownDomain(String),

    #[error("Unknown keyword: {0}")]
    UnknownKeyword(String),

    #[error("Domain not implemented: {0}")]
    NotImplemented(String),

    #[error(transparent)]
    LinearAlgebra(#[from] suma_core::linear_algebra::error::LinearAlgebraError),

    #[error(transparent)]
    Optimization(#[from] suma_core::optimization::error::OptimizationError),
}
