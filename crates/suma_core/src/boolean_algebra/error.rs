use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum BooleanAlgebraError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),

    #[error("Evaluation error: {0}")]
    EvaluationError(#[from] EvaluationError),

    #[error("Invalid expression: {0}")]
    InvalidExpression(#[from] InvalidExpressionError),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseError {
    #[error("Expected '{expected}', found '{found}'")]
    UnexpectedToken { expected: String, found: String },

    #[error("Invalid character: '{0}'")]
    InvalidCharacter(char),

    #[error("Empty expression")]
    EmptyExpression,

    #[error("Invalid operator: '{0}'")]
    InvalidOperator(String),

    #[error("Expected expression: {0}")]
    ExpectedExpression(String),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EvaluationError {
    #[error("Missing variable: '{0}'")]
    MissingVariable(String),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum InvalidExpressionError {
    #[error("Expression too complex (limit: {0} operators)")]
    TooComplex(usize),

    #[error("Invalid variable name: '{0}'")]
    InvalidVariableName(String),
}
