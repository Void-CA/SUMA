// src/core/optimization/linear/transformers/from_symbolics.rs

use crate::symbolics::ast::Expr;
use crate::optimization::linear::model::LinearExpression;
use crate::optimization::linear::error::LinearOptimizationError;

impl LinearExpression {
    /// Intenta convertir una expresión simbólica general en una expresión lineal estricta.
    /// Falla si encuentra no-linealidades (ej: x * y, sin(x)).
    pub fn try_from_ast(expr: &Expr) -> Result<Self, LinearOptimizationError> {
        let mut linear = LinearExpression::new();
        process_node(expr, 1.0, &mut linear)?;
        Ok(linear)
    }
}

// Función auxiliar recursiva para "aplanar" el árbol
fn process_node(
    expr: &Expr, 
    multiplier: f64, 
    acc: &mut LinearExpression
) -> Result<(), LinearOptimizationError> {
    match expr {
        Expr::Const(c) => {
            // Constante * multiplicador acumulado se suma al término independiente
            acc.constant += c * multiplier;
        },
        Expr::Var(name) => {
            // Variable * multiplicador se suma al coeficiente de esa variable
            acc.add_term(name, multiplier);
        },
        Expr::Add(lhs, rhs) => {
            process_node(lhs, multiplier, acc)?;
            process_node(rhs, multiplier, acc)?;
        },
        Expr::Sub(lhs, rhs) => {
            process_node(lhs, multiplier, acc)?;
            process_node(rhs, -multiplier, acc)?; // Note el signo negativo
        },
        Expr::Mul(lhs, rhs) => {
            // AQUÍ está la validación de linealidad.
            // Solo permitimos: Const * Expr  o  Expr * Const
            match (&**lhs, &**rhs) {
                (Expr::Const(c), non_const) | (non_const, Expr::Const(c)) => {
                    process_node(non_const, multiplier * c, acc)?;
                },
                (Expr::Var(_), Expr::Var(_)) => {
                    // Error: Multiplicación de variables (No lineal)
                    return Err(LinearOptimizationError::NonLinearExpression("Variable * Variable detectado".into()));
                },
                _ => {
                    // Casos más complejos requieren simplificación previa
                    return Err(LinearOptimizationError::NonLinearExpression("Multiplicación compleja no soportada".into()));
                }
            }
        },
        Expr::Neg(inner) => {
            process_node(inner, -multiplier, acc)?;
        },
        Expr::Div(lhs, rhs) => {
            // Solo permitimos división por constante
            if let Expr::Const(c) = &**rhs {
                if *c == 0.0 { return Err(LinearOptimizationError::NumericalError("División por cero".into())); }
                process_node(lhs, multiplier / c, acc)?;
            } else {
                return Err(LinearOptimizationError::NonLinearExpression("División por variable".into()));
            }
        }
        // Casos como Pow, Sin, Cos lanzarían error inmediato
        _ => return Err(LinearOptimizationError::NonLinearExpression(format!("Operación no soportada en LP: {:?}", expr))),
    }
    Ok(())
}