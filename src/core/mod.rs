// src/core/mod.rs
pub mod boolean_algebra;
pub mod data_structures;
pub mod conversions;
pub mod matrixes;

// Re-export para fácil acceso
pub use boolean_algebra::{BooleanExpr};
pub use conversions::{NumberConverter};

