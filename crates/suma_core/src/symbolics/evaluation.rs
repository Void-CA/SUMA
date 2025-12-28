use super::error::EvalError;
use super::ast::Expr;
use super::context::Context; // Asegúrate de que Context esté accesible
use std::fmt;



impl Expr {
    /// Evalúa la expresión reduciéndola a un único número f64.
    pub fn evaluate(&self, ctx: &Context) -> Result<f64, EvalError> {
        match self {
            // Caso base: Una constante ya es un número
            Expr::Const(val) => Ok(*val),

            // Caso base: Una variable se busca en el contexto
            Expr::Var(name) => {
                ctx.get(name).ok_or_else(|| EvalError::VariableNotFound(name.clone()))
            },

            // Operaciones recursivas
            Expr::Add(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l + r)
            },
            
            Expr::Sub(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l - r)
            },

            Expr::Mul(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l * r)
            },

            Expr::Div(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                if r == 0.0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(l / r)
            },

            Expr::Neg(expr) => {
                let val = expr.evaluate(ctx)?;
                Ok(-val)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbolics::ast::{var, Expr};
    use crate::symbolics::context::Context;
    use super::*;
    #[test]
fn test_full_evaluation_flow() {

    // 1. Definición del Modelo (Declarativo)
    // Ecuación: y = mx + b
    let m = var("m");
    let x = var("x");
    let b = var("b");
    
    // Representa la expresión "m * x + b"
    let expr = (m * x) + b; 

    // 2. Creación del Contexto (Datos)
    let mut ctx = Context::new();
    ctx.set("m", 2.0);
    ctx.set("x", 3.0);
    ctx.set("b", 1.0);

    // 3. Ejecución (Core Logic)
    let result = expr.evaluate(&ctx);

    assert_eq!(result.unwrap(), 7.0);
}

#[test]
fn test_missing_variable_error() {
    use crate::symbolics::context::Context;

    let expr = var("z") + 5.0;
    let ctx = Context::new(); // Contexto vacío

    match expr.evaluate(&ctx) {
        Err(EvalError::VariableNotFound(name)) => assert_eq!(name, "z"),
        _ => panic!("Debería haber fallado por variable inexistente"),
    }
}
}