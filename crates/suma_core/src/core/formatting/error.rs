use std::error::Error;
use std::fmt;
use csv::{IntoInnerError, Writer};
use rust_xlsxwriter::XlsxError;

// Error personalizado para las operaciones de exportación
#[derive(Debug)]
pub enum ExportError {
    JsonError(serde_json::Error),
    CsvError(csv::Error),
    Utf8Error(std::string::FromUtf8Error),
    IoError(std::io::Error),
    ExcelError(String),
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::JsonError(e) => write!(f, "JSON serialization error: {}", e),
            ExportError::CsvError(e) => write!(f, "CSV serialization error: {}", e),
            ExportError::Utf8Error(e) => write!(f, "UTF-8 conversion error: {}", e),
            ExportError::IoError(e) => write!(f, "I/O error: {}", e),
            ExportError::ExcelError(e) => write!(f, "Excel export error: {}", e),
        }
    }
}

impl Error for ExportError {}

impl From<serde_json::Error> for ExportError {
    fn from(err: serde_json::Error) -> Self {
        ExportError::JsonError(err)
    }
}

impl From<csv::Error> for ExportError {
    fn from(err: csv::Error) -> Self {
        ExportError::CsvError(err)
    }
}

impl From<std::string::FromUtf8Error> for ExportError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ExportError::Utf8Error(err)
    }
}

impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::IoError(err)
    }
}

impl From<IntoInnerError<Writer<Vec<u8>>>> for ExportError {
    fn from(err: IntoInnerError<Writer<Vec<u8>>>) -> Self {
        err.into()
    }
}

impl From<XlsxError> for ExportError {
    fn from(err: XlsxError) -> Self {
        ExportError::ExcelError(err.to_string())
    }
}