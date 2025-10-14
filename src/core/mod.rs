// src/core/mod.rs
pub mod boolean_algebra;
pub mod data_structures;
pub mod conversions;
pub mod matrixes;
pub mod networking;
pub mod decision_theory;

// Re-export para fácil acceso
pub use boolean_algebra::{BooleanExpr, TruthTable};
pub use conversions::{NumberConverter};
pub use networking::{SubnetCalculator};
pub use decision_theory::{decision_tree};

