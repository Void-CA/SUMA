use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("CSV serialization error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Excel export error: {0}")]
    ExcelError(String),
}

impl From<csv::IntoInnerError<csv::Writer<Vec<u8>>>> for ExportError {
    fn from(err: csv::IntoInnerError<csv::Writer<Vec<u8>>>) -> Self {
        ExportError::CsvError(std::io::Error::new(std::io::ErrorKind::Other, err.to_string()).into())
    }
}

impl From<rust_xlsxwriter::XlsxError> for ExportError {
    fn from(err: rust_xlsxwriter::XlsxError) -> Self {
        ExportError::ExcelError(err.to_string())
    }
}
