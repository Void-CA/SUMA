use std::collections::HashMap;

// Importamos el motor matemático
use suma_core::linear_algebra::{
    DenseMatrix,
    systems::LinearSystem,
};
// Trait necesario para operaciones matemáticas
// (Asegúrate de que suma_core exporte Scalar o usar f64 directamente funciona por las impls)

// Importamos el AST de Codex
use crate::domains::linear_algebra::ast::{LinearAlgebraModel, LinAlgStmt, LinAlgExpr};

/// Contexto de ejecución: Mantiene el estado de las variables (Memoria)
pub struct LinearAlgebraExecutor {
    // Almacenamos matrices de f64.
    // En el futuro, esto podría ser un enum Value { Matrix(..), Scalar(..) }
    variables: HashMap<String, DenseMatrix<f64>>,
}

impl LinearAlgebraExecutor {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Punto de entrada: Ejecuta un bloque completo de álgebra lineal
    pub fn execute(&mut self, model: &LinearAlgebraModel) -> Result<(), String> {
        let block_name = model.name.as_deref().unwrap_or("Anónimo");
        println!("--- 🧮 Ejecutando Bloque: {} ---", block_name);

        for stmt in &model.statements {
            match stmt {
                LinAlgStmt::Assignment { target, value } => {
                    let result = self.evaluate(value)?;
                    // Guardamos en memoria
                    self.variables.insert(target.clone(), result);
                    println!("> Variable asignada: '{}'", target);
                },
                LinAlgStmt::Evaluation(expr) => {
                    let result = self.evaluate(expr)?;
                    // Imprimimos el resultado (DenseMatrix implementa Display)
                    println!("> Resultado:\n{}", result);
                }
            }
        }
        Ok(())
    }

    /// Evaluador Recursivo: Convierte AST -> DenseMatrix<f64>
    fn evaluate(&self, expr: &LinAlgExpr) -> Result<DenseMatrix<f64>, String> {
        match expr {
            // 1. Literal Numérico: Lo convertimos en Matriz 1x1
            LinAlgExpr::Number(n) => {
                Ok(DenseMatrix::new(1, 1, vec![*n]))
            },

            // 2. Variable: Buscamos en el HashMap
            LinAlgExpr::Variable(name) => {
                self.variables.get(name)
                    .cloned() // Clonamos porque DenseMatrix es dueña de sus datos
                    .ok_or_else(|| format!("Error: La variable '{}' no está definida.", name))
            },

            // 3. Literal de Matriz: [1, 2; 3, 4]
            LinAlgExpr::Matrix(rows_ast) => {
                if rows_ast.is_empty() {
                    return Err("Matriz vacía no permitida".to_string());
                }

                let num_rows = rows_ast.len();
                let num_cols = rows_ast[0].len(); // Asumimos regularidad inicial
                let mut flat_data = Vec::with_capacity(num_rows * num_cols);

                for (i, row) in rows_ast.iter().enumerate() {
                    if row.len() != num_cols {
                        return Err(format!("Fila {} tiene longitud distinta. Se esperaban {} columnas.", i, num_cols));
                    }
                    for cell_expr in row {
                        // Recursividad: Una celda puede ser una expresión "1 + 1"
                        let cell_matrix = self.evaluate(cell_expr)?;

                        // Restricción V1: Solo permitimos escalares dentro de matrices
                        if cell_matrix.rows == 1 && cell_matrix.cols == 1 {
                            flat_data.push(cell_matrix.get(0, 0));
                        } else {
                            return Err("No se permiten matrices anidadas. La celda debe evaluar a un escalar.".to_string());
                        }
                    }
                }

                Ok(DenseMatrix::new(num_rows, num_cols, flat_data))
            },

            // 4. Operaciones Binarias (+, -, *, /)
            LinAlgExpr::BinaryOp { lhs, op,rhs } => {
                let l = self.evaluate(lhs)?;
                let r = self.evaluate(rhs)?;

                match op.as_str() {
                    "+" => Ok(&l + &r), // Usa la impl Add de suma_core
                    "-" => Ok(&l - &r), // Usa la impl Sub de suma_core
                    "*" => (&l * &r).map_err(|e| e.to_string()), // Usa la impl Mul (Matricial) de suma_core
                    "/" => Err("La división matricial no está definida estándar. Usa inv(B) * A".to_string()),
                    _ => Err(format!("Operador desconocido: {}", op)),
                }
            },

            // 5. Llamadas a Funciones (El puente a tus algoritmos)
            LinAlgExpr::Call { function, args } => {
                // Evaluamos todos los argumentos primero
                let mut eval_args = Vec::new();
                for arg in args {
                    eval_args.push(self.evaluate(arg)?);
                }

                match function.as_str() {
                    "solve" => {
                        if eval_args.len() != 2 { return Err("solve(A, b) requiere 2 argumentos".into()); }
                        // LLAMADA A TU CORE: LinearSystem
                        LinearSystem::solve(&eval_args[0], &eval_args[1])
                            .map_err(|e| e.to_string()) // Convertimos LinearAlgebraError a String
                    },
                    "inv" | "inverse" => {
                        if eval_args.len() != 1 { return Err("inv(A) requiere 1 argumento".into()); }
                        // LLAMADA A TU CORE: inverse()
                        eval_args[0].inverse()
                            .map_err(|e| e.to_string())
                    },
                    "det" | "determinant" => {
                        if eval_args.len() != 1 { return Err("det(A) requiere 1 argumento".into()); }
                        // LLAMADA A TU CORE: determinant()
                        let val = eval_args[0].determinant().map_err(|e| e.to_string())?;
                        // Devolvemos como matriz 1x1
                        Ok(DenseMatrix::new(1, 1, vec![val]))
                    },
                    "transpose" | "t" => {
                        // Asumiendo que implementaste transpose, si no, es buen momento para agregarlo al core
                        // eval_args[0].transpose()
                        Err("Función transpose no implementada en adapter aun".to_string())
                    }
                    _ => Err(format!("Función desconocida: '{}'", function)),
                }
            }
        }
    }
}