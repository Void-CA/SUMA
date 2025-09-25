// src/core/boolean_algebra/mod.rs
pub mod ast;
pub mod parser;
pub mod boolean_expr;
pub mod error;  // NUEVO

// Re-export para fácil acceso
pub use ast::Node;
pub use parser::{parse_expression, tokenize};
pub use boolean_expr::BooleanExpr;
pub use error::{BooleanAlgebraError, ParseError, EvaluationError, InvalidExpressionError};  // NUEVO

// Tipo Result personalizado para todo el módulo
pub type Result<T> = std::result::Result<T, BooleanAlgebraError>;