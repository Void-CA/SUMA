#[derive(Debug, thiserror::Error)]
pub enum SumaError {
    #[error(transparent)]
    Network(#[from] crate::networking::NetworkError),

    #[error(transparent)]
    Export(#[from] crate::formatting::error::ExportError)
}