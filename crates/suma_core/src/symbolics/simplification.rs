use super::ast::Expr;

impl Expr {
    /// Reduce la expresión aplicando reglas algebraicas y aritméticas básicas.
    /// Devuelve una nueva expresión simplificada.
    pub fn simplify(&self) -> Expr {
        match self {
            // Casos base: Constantes y Variables ya son simples
            Expr::Const(_) | Expr::Var(_) => self.clone(),

            Expr::Add(lhs, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();
                
                match (l, r) {
                    // 1. Constant Folding
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(c1 + c2),
                    
                    // 2. Identidad Aditiva
                    (expr, Expr::Const(c)) if c == 0.0 => expr,
                    (Expr::Const(c), expr) if c == 0.0 => expr,
                    
                    // 3. Cancelación de Opuestos (Corregido)
                    // a + (-a) = 0
                    (a, Expr::Neg(b_inner)) if a == *b_inner => Expr::Const(0.0),
                    // (-a) + a = 0
                    (Expr::Neg(a_inner), b) if *a_inner == b => Expr::Const(0.0),

                    // Default
                    (new_l, new_r) => Expr::Add(Box::new(new_l), Box::new(new_r)),
                }
            },

            Expr::Sub(lhs, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();

                match (l, r) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(c1 - c2),
                    // x - 0 = x
                    (expr, Expr::Const(c)) if c == 0.0 => expr,
                    // 0 - x = -x
                    (Expr::Const(c), expr) if c == 0.0 => Expr::Neg(Box::new(expr)),
                    // x - x = 0
                    (l_expr, r_expr) if l_expr == r_expr => Expr::Const(0.0),
                    
                    // --- NUEVO: Manejo de Resta de Negativos ---
                    // a - (-b) -> a + b
                    (a, Expr::Neg(b)) => Expr::Add(Box::new(a), b).simplify(),

                    (new_l, new_r) => Expr::Sub(Box::new(new_l), Box::new(new_r)),
                }
            },

            Expr::Mul(lhs, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();

                match (l, r) {
                    (Expr::Const(c1), Expr::Const(c2)) => Expr::Const(c1 * c2),
                    // x * 0 = 0
                    (_, Expr::Const(c)) if c == 0.0 => Expr::Const(0.0),
                    (Expr::Const(c), _) if c == 0.0 => Expr::Const(0.0),
                    // x * 1 = x
                    (expr, Expr::Const(c)) if c == 1.0 => expr,
                    (Expr::Const(c), expr) if c == 1.0 => expr,

                    // --- NUEVO: Canonicalización de Signos ---
                    // x * -1 -> -x
                    (expr, Expr::Const(c)) if c == -1.0 => Expr::Neg(Box::new(expr)),
                    (Expr::Const(c), expr) if c == -1.0 => Expr::Neg(Box::new(expr)),
                    
                    // (-a) * (-b) -> a * b
                    (Expr::Neg(a), Expr::Neg(b)) => Expr::Mul(a, b),
                    
                    // (-a) * b -> -(a * b)  (Extraer signo)
                    (Expr::Neg(a), b) => Expr::Neg(Box::new(Expr::Mul(a, Box::new(b)))),
                    (a, Expr::Neg(b)) => Expr::Neg(Box::new(Expr::Mul(Box::new(a), b))),

                    (new_l, new_r) => Expr::Mul(Box::new(new_l), Box::new(new_r)),
                }
            },

            Expr::Div(lhs, rhs) => {
                let l = lhs.simplify();
                let r = rhs.simplify();

                match (l, r) {
                    (Expr::Const(c1), Expr::Const(c2)) => {
                        if c2 == 0.0 {
                            Expr::Div(Box::new(Expr::Const(c1)), Box::new(Expr::Const(c2)))
                        } else {
                            Expr::Const(c1 / c2)
                        }
                    },
                    // 0 / x = 0
                    (Expr::Const(c), _) if c == 0.0 => Expr::Const(0.0),
                    // x / 1 = x
                    (expr, Expr::Const(c)) if c == 1.0 => expr,
                    // x / x = 1
                    (l_expr, r_expr) if l_expr == r_expr => Expr::Const(1.0),

                    // --- NUEVO: Signos en División ---
                    // (-a) / (-b) -> a / b
                    (Expr::Neg(a), Expr::Neg(b)) => Expr::Div(a, b),
                    
                    // (-a) / b -> -(a / b)
                    (Expr::Neg(a), b) => Expr::Neg(Box::new(Expr::Div(a, Box::new(b)))),
                    // a / (-b) -> -(a / b)
                    (a, Expr::Neg(b)) => Expr::Neg(Box::new(Expr::Div(Box::new(a), b))),

                    (new_l, new_r) => Expr::Div(Box::new(new_l), Box::new(new_r)),
                }
            },

            Expr::Neg(inner) => {
                let i = inner.simplify();
                match i {
                    Expr::Const(c) => Expr::Const(-c),
                    // -(-x) = x
                    Expr::Neg(deep_inner) => *deep_inner,
                    new_inner => Expr::Neg(Box::new(new_inner)),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolics::ast::{var, Expr};

    #[test]
    fn test_constant_folding() {
        // (2 + 3) * 4 -> 5 * 4 -> 20
        let expr = (Expr::from(2.0) + 3.0) * 4.0;
        let simplified = expr.simplify();
        assert_eq!(simplified, Expr::Const(20.0));
    }

    #[test]
    fn test_additive_identity() {
        // x + 0 -> x
        let expr = var("x") + 0.0;
        let simplified = expr.simplify();
        assert_eq!(simplified, var("x"));
    }

    #[test]
    fn test_multiplicative_zero() {
        // (x + y) * 0 -> 0
        let expr = (var("x") + var("y")) * 0.0;
        let simplified = expr.simplify();
        assert_eq!(simplified, Expr::Const(0.0));
    }

    #[test]
    fn test_complex_reduction() {
        // 1 * x + (3 - 3) -> x + 0 -> x
        let expr = (Expr::from(1.0) * var("x")) + (Expr::from(3.0) - 3.0);
        let simplified = expr.simplify();
        assert_eq!(simplified, var("x"));
    }
}