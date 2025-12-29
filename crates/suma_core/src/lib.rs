// src/lib.rs

#![allow(warnings)]

pub mod boolean_algebra;
pub mod data_structures;
pub mod conversions;

pub mod networking;
pub mod decision_theory;
pub mod formatting;

pub mod error;
pub mod probability;
pub mod linear_algebra;
pub mod optimization;

pub mod symbolics;

// Re-export para fácil acceso
pub use boolean_algebra::{BooleanExpr, TruthTable};
pub use conversions::{NumberConverter};



pub use networking::subnets::{FLSMCalculator, SubnetRow, VLSMCalculator, BaseCalculator};
pub use networking::utils::ipv6_format::{compress_ipv6, expand_ipv6};

