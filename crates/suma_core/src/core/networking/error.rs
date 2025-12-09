#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Dirección IP inválida: {0}")]
    InvalidAddress(String),

    #[error("Timeout de conexión")]
    Timeout,

    #[error("Error de socket: {0}")]
    Socket(String),
}
