// ── Numerical tolerances ──

/// General comparison tolerance for floating-point equality checks.
pub const EPSILON: f64 = 1e-9;

/// Tolerance for detecting non-zero values (e.g., pivot elements).
pub const ZERO_TOLERANCE: f64 = 1e-12;

/// Tolerance for infeasibility detection in simplex Phase I.
pub const INFEASIBILITY_TOLERANCE: f64 = 1e-5;

/// Tolerance for float comparisons in constraints and bounds.
pub const CONSTRAINT_TOLERANCE: f64 = 1e-9;

// ── Algorithm limits ──

/// Maximum iterations for simplex and similar iterative algorithms.
pub const MAX_SIMPLEX_ITERATIONS: usize = 10000;
