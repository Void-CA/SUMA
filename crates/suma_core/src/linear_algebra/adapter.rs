use crate::{linear_algebra::Scalar, symbolics::ast::Expr}; // Importamos tu módulo anterior
use super::traits::{Zero, One};  // Importamos los traits nuevos

// --- Adaptador para f64 ---
impl Zero for f64 {
    fn zero() -> Self { 0.0 }
    fn is_zero(&self) -> bool { *self == 0.0 }
}
impl One for f64 {
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn is_approx(&self, other: &Self) -> bool {
        // Aquí está la MAGIA que arregla tu test:
        // Usamos un Epsilon (margen de error) de 1e-10
        (self - other).abs() < 1e-10
    }
}

// --- Adaptador para Expr ---
impl Zero for Expr {
    fn zero() -> Self { Expr::Const(0.0) }
    fn is_zero(&self) -> bool {
        // Ojo: Aquí podrías usar tu simplify() o evaluar si es Const(0.0)
        matches!(self, Expr::Const(c) if *c == 0.0)
    }
}
impl One for Expr {
    fn one() -> Self { Expr::Const(1.0) }
}

impl Scalar for Expr {
    fn is_approx(&self, other: &Self) -> bool {
        // Para símbolos, "aproximado" significa "estructuralmente idéntico".
        // x es x. x no es aproximadamente y.
        // Esto usa el PartialEq que derivaste en ast.rs
        self == other
    }
}