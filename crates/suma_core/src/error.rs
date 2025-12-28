use core::error;

#[derive(Debug, thiserror::Error)]
pub enum SumaError {
    #[error(transparent)]
    Network(#[from] crate::networking::NetworkError),

    #[error(transparent)]
    Export(#[from] crate::formatting::error::ExportError),

    #[error(transparent)]
    SymbolicsEval(#[from] crate::symbolics::error::EvalError),

    #[error(transparent)]
    LinearAlgebra(#[from] crate::linear_algebra::error::LinearAlgebraError),
}