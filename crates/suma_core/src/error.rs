#[derive(Debug, thiserror::Error)]
pub enum SumaError {
    #[error(transparent)]
    Network(#[from] crate::networking::NetworkError),

    #[error(transparent)]
    Export(#[from] crate::formatting::error::ExportError),

    #[error(transparent)]
    BooleanAlgebra(#[from] crate::boolean_algebra::error::BooleanAlgebraError),

    #[error(transparent)]
    SymbolicsEval(#[from] crate::symbolics::error::EvalError),

    #[error(transparent)]
    LinearAlgebra(#[from] crate::linear_algebra::error::LinearAlgebraError),

    #[error(transparent)]
    Optimization(#[from] crate::optimization::error::OptimizationError),
}