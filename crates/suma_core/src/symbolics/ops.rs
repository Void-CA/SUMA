use std::ops::{Add, Sub, Mul, Div, Neg};
use super::ast::Expr; // Importamos la definición del AST

// 1. Permitir convertir f64 a Expr automáticamente
// Esto nos deja sumar variables con números directamente.
impl From<f64> for Expr {
    fn from(n: f64) -> Self {
        Expr::Const(n)
    }
}

// 2. Macro para implementar operaciones binarias (Add, Sub, Mul, Div)
// Esto evita escribir 4 veces la misma lógica de "Box::new(...)".
macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $variant:ident) => {
        impl $trait<Expr> for Expr {
            type Output = Expr;
            fn $method(self, rhs: Expr) -> Self::Output {
                Expr::$variant(Box::new(self), Box::new(rhs))
            }
        }

        // Sobrecarga para: Expr + f64
        impl $trait<f64> for Expr {
            type Output = Expr;
            fn $method(self, rhs: f64) -> Self::Output {
                Expr::$variant(Box::new(self), Box::new(Expr::from(rhs)))
            }
        }
        
        // Nota: Para f64 + Expr se requiere un poco más de "magia" en Rust 
        // (newtypes) que podemos ver luego si es necesario.
    };
}

// Aplicamos la macro para las 4 operaciones básicas
impl_binary_op!(Add, add, Add);
impl_binary_op!(Sub, sub, Sub);
impl_binary_op!(Mul, mul, Mul);
impl_binary_op!(Div, div, Div);

// 3. Operador Unario (Negación: -x)
impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Self::Output {
        Expr::Neg(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolics::ast::var;

    use super::*;
    
    #[test]
    fn test_add_expr() {
        let expr1 = var("x") + 5.0;
        let expr2 = Expr::Add(Box::new(Expr::Var("x".to_string())), Box::new(Expr::Const(5.0)));
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_sub_expr() {
        let expr1 = var("y") - 3.0;
        let expr2 = Expr::Sub(Box::new(Expr::Var("y".to_string())), Box::new(Expr::Const(3.0)));
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_mul_expr() {
        let expr1 = var("z") * 2.0;
        let expr2 = Expr::Mul(Box::new(Expr::Var("z".to_string())), Box::new(Expr::Const(2.0)));
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_div_expr() {
        let expr1 = var("a") / 4.0;
        let expr2 = Expr::Div(Box::new(Expr::Var("a".to_string())), Box::new(Expr::Const(4.0)));
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_neg_expr() {
        let expr1 = -var("b");
        let expr2 = Expr::Neg(Box::new(Expr::Var("b".to_string())));
        assert_eq!(expr1, expr2);
    }
}