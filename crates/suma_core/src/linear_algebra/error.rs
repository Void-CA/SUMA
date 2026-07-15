use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum LinearAlgebraError {
    #[error("Dimension mismatch in {operation}: expected {expected}, found {found}")]
    DimensionMismatch {
        operation: String,
        expected: usize,
        found: usize,
    },

    #[error("Index out of bounds: {context} at index {index}, max is {max}")]
    IndexOutOfBounds {
        context: String,
        index: usize,
        max: usize,
    },
}
