use super::ast::Expr;

impl Expr {
    /// Reemplaza todas las ocurrencias de una variable `target_var` 
    /// por una nueva expresión `replacement`.
    pub fn substitute(&self, target_var: &str, replacement: &Expr) -> Expr {
        match self {
            // Caso Base 1: Constantes no cambian
            Expr::Const(_) => self.clone(),

            // Caso Base 2: Variable
            Expr::Var(name) => {
                if name == target_var {
                    // Devolver reemplazo
                    replacement.clone()
                } else {
                    // No es la variable que buscamos
                    self.clone()
                }
            },

            // Operaciones Recursivas: "Bajar" por el árbol buscando la variable
            Expr::Add(lhs, rhs) => Expr::Add(
                Box::new(lhs.substitute(target_var, replacement)),
                Box::new(rhs.substitute(target_var, replacement)),
            ),

            Expr::Sub(lhs, rhs) => Expr::Sub(
                Box::new(lhs.substitute(target_var, replacement)),
                Box::new(rhs.substitute(target_var, replacement)),
            ),

            Expr::Mul(lhs, rhs) => Expr::Mul(
                Box::new(lhs.substitute(target_var, replacement)),
                Box::new(rhs.substitute(target_var, replacement)),
            ),

            Expr::Div(lhs, rhs) => Expr::Div(
                Box::new(lhs.substitute(target_var, replacement)),
                Box::new(rhs.substitute(target_var, replacement)),
            ),

            Expr::Neg(inner) => Expr::Neg(
                Box::new(inner.substitute(target_var, replacement))
            ),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::symbolics::ast::{var, Expr};

    #[test]
    fn test_simple_substitution() {
        // Expresión: x + 2
        // Sustituir x = 5
        // Resultado esperado: 5 + 2 (Nota: no 7, porque no simplificamos aquí)
        let expr = var("x") + 2.0;
        let replacement = Expr::from(5.0);
        
        let result = expr.substitute("x", &replacement);
        
        // Verificamos estructura: Add(Const(5.0), Const(2.0))
        let expected = Expr::Add(Box::new(Expr::Const(5.0)), Box::new(Expr::Const(2.0)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_symbolic_substitution() {
        // Escenario de Sistema de Ecuaciones:
        // Eq 1: z = 2 * y
        // Eq 2: x + z
        // Queremos reemplazar z en Eq 2 con (2 * y)
        
        let eq2 = var("x") + var("z"); // x + z
        let val_z = var("y") * 2.0;    // y * 2
        
        // Sustituimos z -> (y * 2)
        let result = eq2.substitute("z", &val_z);
        
        // Esperamos: x + (y * 2)
        match result {
            Expr::Add(lhs, rhs) => {
                assert_eq!(*lhs, var("x"));
                // El lado derecho debe ser el árbol insertado
                match *rhs {
                    Expr::Mul(mul_lhs, mul_rhs) => {
                        assert_eq!(*mul_lhs, var("y"));
                        assert_eq!(*mul_rhs, Expr::Const(2.0));
                    },
                    _ => panic!("Estructura incorrecta en RHS"),
                }
            },
            _ => panic!("Estructura raíz incorrecta"),
        }
    }
    
    #[test]
    fn test_chaining_simplify() {
        // Prueba de integración: Sustituir y luego Simplificar
        // P(x) = 2x + 1. Evaluar en x=3.
        let polynomial = (var("x") * 2.0) + 1.0;
        
        let substituted = polynomial.substitute("x", &Expr::from(3.0)); // (3 * 2) + 1
        let final_val = substituted.simplify(); // 7
        
        assert_eq!(final_val, Expr::Const(7.0));
    }
}