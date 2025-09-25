// src/core/boolean_algebra/error.rs
use std::fmt;

/// Error principal del módulo de álgebra booleana
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanAlgebraError {
    ParseError(ParseError),
    EvaluationError(EvaluationError),
    InvalidExpression(InvalidExpressionError),
}

/// Errores específicos de parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    UnclosedParenthesis,
    InvalidCharacter(char),
    EmptyExpression,
    InvalidOperator(String),
    ExpectedExpression(String),
}

/// Errores de evaluación
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationError {
    MissingVariable(String),
    InvalidVariableType(String),
    CircularReference(String),
}

/// Errores de expresiones inválidas
#[derive(Debug, Clone, PartialEq)]
pub enum InvalidExpressionError {
    TooComplex(usize), // Límite de complejidad
    UnsupportedOperation(String),
    InvalidVariableName(String),
}

// Implementaciones de Display para errores amigables
impl fmt::Display for BooleanAlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BooleanAlgebraError::ParseError(e) => write!(f, "Error de análisis: {}", e),
            BooleanAlgebraError::EvaluationError(e) => write!(f, "Error de evaluación: {}", e),
            BooleanAlgebraError::InvalidExpression(e) => write!(f, "Expresión inválida: {}", e),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "Se esperaba '{}' pero se encontró '{}'", expected, found)
            }
            ParseError::UnclosedParenthesis => {
                write!(f, "Paréntesis sin cerrar")
            }
            ParseError::InvalidCharacter(c) => {
                write!(f, "Carácter inválido: '{}'", c)
            }
            ParseError::EmptyExpression => {
                write!(f, "Expresión vacía")
            }
            ParseError::InvalidOperator(op) => {
                write!(f, "Operador inválido: '{}'", op)
            }
            ParseError::ExpectedExpression(context) => {
                write!(f, "Se esperaba una expresión: {}", context)
            }
        }
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::MissingVariable(var) => {
                write!(f, "Variable faltante: '{}'", var)
            }
            EvaluationError::InvalidVariableType(var) => {
                write!(f, "Tipo inválido para variable '{}'", var)
            }
            EvaluationError::CircularReference(ref_desc) => {
                write!(f, "Referencia circular: {}", ref_desc)
            }
        }
    }
}

impl fmt::Display for InvalidExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidExpressionError::TooComplex(limit) => {
                write!(f, "Expresión demasiado compleja (límite: {} operadores)", limit)
            }
            InvalidExpressionError::UnsupportedOperation(op) => {
                write!(f, "Operación no soportada: '{}'", op)
            }
            InvalidExpressionError::InvalidVariableName(name) => {
                write!(f, "Nombre de variable inválido: '{}'", name)
            }
        }
    }
}

// Conversiones automáticas para facilitar el uso
impl From<ParseError> for BooleanAlgebraError {
    fn from(error: ParseError) -> Self {
        BooleanAlgebraError::ParseError(error)
    }
}

impl From<EvaluationError> for BooleanAlgebraError {
    fn from(error: EvaluationError) -> Self {
        BooleanAlgebraError::EvaluationError(error)
    }
}

impl From<InvalidExpressionError> for BooleanAlgebraError {
    fn from(error: InvalidExpressionError) -> Self {
        BooleanAlgebraError::InvalidExpression(error)
    }
}

// Implementación de std::error::Error para compatibilidad
impl std::error::Error for BooleanAlgebraError {}
impl std::error::Error for ParseError {}
impl std::error::Error for EvaluationError {}
impl std::error::Error for InvalidExpressionError {}