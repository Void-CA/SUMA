#[derive(Debug, thiserror::Error)]
pub enum SumaError {
    #[error(transparent)]
    Network(#[from] crate::core::networking::NetworkError),

    #[error(transparent)]
    Export(#[from] crate::core::formatting::error::ExportError)
}