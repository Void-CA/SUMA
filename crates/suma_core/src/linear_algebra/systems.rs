use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
use crate::linear_algebra::traits::Scalar;
use crate::linear_algebra::error::LinearAlgebraError;

/// Estructura helper para resolver sistemas de ecuaciones lineales.
/// No almacena datos, solo provee métodos estáticos de utilidad.
pub struct LinearSystem;

impl LinearSystem {
    /// Resuelve el sistema lineal Ax = b utilizando Eliminación Gaussiana (RREF).
    /// 
    /// # Argumentos
    /// * `a` - Matriz de coeficientes (NxN)
    /// * `b` - Vector de constantes (Nx1)
    /// 
    /// # Retorna
    /// * `Result<DenseMatrix<T>, ...>` - El vector solución x (Nx1) o error.
    pub fn solve<T>(a: &DenseMatrix<T>, b: &DenseMatrix<T>) -> Result<DenseMatrix<T>, LinearAlgebraError>
    where
        T: Scalar,
    {
        // 1. Validaciones
        if a.rows != a.cols {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Solve System (A must be square)".to_string(),
                expected: a.rows,
                found: a.cols,
            });
        }
        if a.rows != b.rows {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Solve System (Rows A vs Rows b)".to_string(),
                expected: a.rows,
                found: b.rows,
            });
        }
        if b.cols != 1 {
            return Err(LinearAlgebraError::DimensionMismatch {
                operation: "Solve System (b must be a vector)".to_string(),
                expected: 1,
                found: b.cols,
            });
        }

        // 2. Construir Matriz Aumentada [A | b] manualmente
        // (Sería ideal tener un método a.augment(&b) en DenseMatrix en el futuro)
        let rows = a.rows;
        let cols_aug = a.cols + 1;
        let mut aug_data = Vec::with_capacity(rows * cols_aug);

        for i in 0..rows {
            // Copiar fila de A
            for j in 0..a.cols {
                aug_data.push(a.get(i, j));
            }
            // Copiar elemento de b
            aug_data.push(b.get(i, 0));
        }

        let mut augmented = DenseMatrix::new(rows, cols_aug, aug_data);

        // 3. Resolver usando el motor existente
        augmented.rref()?;

        // 4. Extraer solución y validar consistencia
        let mut x_data = Vec::with_capacity(rows);
        
        for i in 0..rows {
            // Verificar si tenemos un pivote válido en la diagonal
            // En RREF de una matriz invertible, la diagonal debe ser 1.
            let diag = augmented.get(i, i);
            if diag.is_zero() {
                // Si la diagonal es 0, el sistema es singular (inconsistente o infinitas soluciones)
                // Para una solver simple, esto es un error.
                 return Err(LinearAlgebraError::DimensionMismatch {
                    operation: "Solve System (Singular Matrix)".to_string(),
                    expected: 1,
                    found: 0,
                });
            }
            
            // La solución está en la última columna
            x_data.push(augmented.get(i, rows));
        }

        Ok(DenseMatrix::new(rows, 1, x_data))
    }
}

#[cfg(test)]
mod tests {
    use std::task::Context;

    use super::*;
    use crate::linear_algebra::matrices::implementations::dense::DenseMatrix;
    use crate::matrix;
    use crate::symbolics::ast::{var, Expr};
    use crate::symbolics::context;
    // Asumimos que el macro matrix! está disponible. 
    // Si no compila, añade `use crate::matrix;` o asegúrate de que sea #[macro_export]
    
    #[test]
    fn test_solve_numeric_system_2x2() {
        // Sistema:
        // 2x + y = 5
        // x + 3y = 5
        // Solución esperada: x=2, y=1
        
        let a = matrix![
            2.0, 1.0;
            1.0, 3.0
        ];
        
        let b = matrix![
            5.0;
            5.0
        ];

        let x = LinearSystem::solve(&a, &b).expect("El sistema tiene solucion única");

        let expected = matrix![
            2.0;
            1.0
        ];

        // Usamos is_approx para evitar errores de punto flotante
        if !x.is_approx(&expected) {
            panic!("Error numérico.\nObtenido: {:?}\nEsperado: {:?}", x, expected);
        }
    }

    #[test]
    fn test_solve_identity_system() {
        // I * x = b  =>  x = b
        let a = matrix![
            1.0, 0.0, 0.0;
            0.0, 1.0, 0.0;
            0.0, 0.0, 1.0
        ];
        
        let b = matrix![
            10.0;
            20.0;
            30.0
        ];

        let x = LinearSystem::solve(&a, &b).expect("Solución trivial");
        assert!(x.is_approx(&b));
    }

    #[test]
    fn test_solve_singular_system_error() {
        // Sistema sin solución única (Filas dependientes)
        // [ 1  2 ]
        // [ 2  4 ] 
        // Determinante es 0.
        let a = matrix![
            1.0, 2.0;
            2.0, 4.0
        ];
        
        let b = matrix![
            3.0;
            6.0
        ];

        let result = LinearSystem::solve(&a, &b);

        match result {
            Err(LinearAlgebraError::DimensionMismatch { operation, .. }) => {
                // Verificamos que sea el error de Matriz Singular que definimos
                assert!(operation.contains("Singular Matrix"));
            },
            _ => panic!("Debería fallar por ser matriz singular, obtuvo: {:?}", result),
        }
    }

    #[test]
    fn test_solve_dimension_mismatch_error() {
        // A es 2x2
        let a = matrix![
            1.0, 2.0;
            3.0, 4.0
        ];
        
        // b es 3x1 (Incompatible)
        let b = matrix![
            1.0;
            2.0;
            3.0
        ];

        let result = LinearSystem::solve(&a, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_symbolic_system() {
        // Sistema Simbólico Triangular Superior:
        // [ 1  a ] [ x ]   [ 0 ]  ->  1*x + a*y = 0  -> x = -ab
        // [ 0  1 ] [ y ] = [ b ]  ->  0*x + 1*y = b  -> y = b
        
        let val_a = var("a");
        let val_b = var("b");

        let a = matrix![
            Expr::from(1.0), val_a.clone();
            Expr::from(0.0), Expr::from(1.0)
        ];

        let b = matrix![
            Expr::from(0.0);
            val_b.clone()
        ];

        let x_vec = LinearSystem::solve(&a, &b).expect("Sistema simbólico válido");

        // 1. Verificar 'y' (posición 1,0 en vector solución)
        // Debe ser 'b'
        let y_res = x_vec.get(1, 0).simplify();
        assert_eq!(y_res, val_b);

        // 2. Verificar 'x' (posición 0,0 en vector solución)
        // Debe ser '-a * b' (o '0 - a*b')
        let x_res = x_vec.get(0, 0).simplify();
        
        // Construimos la expectativa manual: -(a * b)
        let expected_x = - (val_a * val_b);
        // Ojo: Dependiendo de tu simplificador, podría ser 0 - (a*b) o -(a*b).
        // Si tienes problemas aquí, imprime x_res para ver cómo simplificó.
        
        // Hacemos una validación estructural básica si la exacta falla por orden:
        match x_res {
            Expr::Neg(inner) => {
                // Esperamos Mul(a, b) dentro del Neg
                if let Expr::Mul(lhs, rhs) = *inner {
                    // Verificar que sean a y b (en cualquier orden)
                    let is_ab = (*lhs == var("a") && *rhs == var("b"));
                    let is_ba = (*lhs == var("b") && *rhs == var("a"));
                    assert!(is_ab || is_ba, "Estructura interna incorrecta");
                } else {
                    panic!("Se esperaba una multiplicación dentro del negativo");
                }
            },
            // Aceptamos también Add(0, Neg(...)) si el simplificador no quitó el 0 inicial
            _ => {
                 // Opción B: Evaluar numéricamente para estar seguros
                 // Si a=2, b=3 -> x debería ser -6
                 let a = var("a");
                 let b = var("b");

                 let mut ctx = crate::symbolics::Context::new();
                 ctx.set("a", 2.0);
                 ctx.set("b", 3.0);


                 
                 let val = x_res.evaluate(&ctx).expect("Evaluación numérica");
                 assert_eq!(val, -6.0);
            }
        }
    }
}