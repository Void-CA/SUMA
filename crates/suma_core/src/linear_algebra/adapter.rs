use crate::symbolics::ast::Expr; // Importamos tu módulo anterior
use super::traits::{Zero, One};  // Importamos los traits nuevos

// --- Adaptador para f64 ---
impl Zero for f64 {
    fn zero() -> Self { 0.0 }
    fn is_zero(&self) -> bool { *self == 0.0 }
}
impl One for f64 {
    fn one() -> Self { 1.0 }
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
