// src/core/optimization/linear/algorithms/simplex.rs

use super::super::internal::tableau::SimplexTableau;
use super::super::transformers::standard_form::{to_standard_form, StandardFormResult};
use crate::optimization::linear::model::LinearProblem;
use crate::optimization::linear::error::{OptimizationResult, OptimizationError, Solution, OptimizationStatus};
use std::collections::HashMap;

const MAX_ITERATIONS: usize = 10000;

pub fn solve_primal(problem: &LinearProblem) -> OptimizationResult {
    // 1. Convertir modelo de usuario a Forma Estándar
    // Nota: to_standard_form devuelve un struct, no una tupla
    let StandardFormResult { mut tableau, reverse_map, .. } = to_standard_form(problem)
        .map_err(|e| OptimizationError::ValidationError(format!("{:?}", e)))?;

    let mut iterations = 0;

    // 2. Iterar hasta optimidad
    loop {
        if iterations >= MAX_ITERATIONS {
            return Err(OptimizationError::MaxIterationsReached);
        }
        iterations += 1;

        if is_optimal(&tableau) {
            return extract_solution(&tableau, &reverse_map);
        }
        
        // Regla de Dantzig: Elegir columna pivote (variable que entra)
        // Si retorna None, es óptimo (pero ya lo checamos arriba)
        let pivot_col = match select_entering_variable(&tableau) {
            Some(col) => col,
            None => return extract_solution(&tableau, &reverse_map), 
        };

        // Test del Cociente Mínimo: Elegir fila pivote (variable que sale)
        let pivot_row = match select_leaving_variable(&tableau, pivot_col) {
            Some(row) => row,
            None => return Err(OptimizationError::Unbounded), // Si no hay fila que limite, es infinito
        };
        
        // Ejecutar operación de pivoteo sobre la matriz
        tableau.pivot(pivot_row, pivot_col);
    }
}

// --- Funciones Auxiliares (Placeholders lógicos) ---

fn is_optimal(tableau: &SimplexTableau) -> bool {
    // En maximización estándar, si todos los coeficientes de la fila Z son >= 0, es óptimo.
    // Esto depende de tu convención de signos en standard_form.
    // Asumiremos convención: Z - c*x = 0. Coeficientes negativos indican mejora posible.
    
    let last_row_idx = tableau.matrix.rows - 1;
    for j in 0..(tableau.matrix.cols - 1) { // -1 para ignorar RHS
        if tableau.matrix.get(last_row_idx, j) < -1e-9 {
            return false;
        }
    }
    true
}

fn select_entering_variable(tableau: &SimplexTableau) -> Option<usize> {
    // Seleccionar el coeficiente más negativo de la fila Z
    let last_row_idx = tableau.matrix.rows - 1;
    let mut min_val = 0.0;
    let mut entering_col = None;

    for j in 0..(tableau.matrix.cols - 1) {
        let val = tableau.matrix.get(last_row_idx, j);
        if val < min_val {
            min_val = val;
            entering_col = Some(j);
        }
    }
    entering_col
}

fn select_leaving_variable(tableau: &SimplexTableau, col_idx: usize) -> Option<usize> {
    // Ratio test: RHS / Coeff para Coeff > 0
    let mut min_ratio = f64::INFINITY;
    let mut leaving_row = None;

    for i in 0..(tableau.matrix.rows - 1) { // Excluir fila Z
        let coeff = tableau.matrix.get(i, col_idx);
        let rhs = tableau.matrix.get(i, tableau.matrix.cols - 1);

        if coeff > 1e-9 {
            let ratio = rhs / coeff;
            if ratio < min_ratio {
                min_ratio = ratio;
                leaving_row = Some(i);
            }
        }
    }
    leaving_row
}

fn extract_solution(tableau: &SimplexTableau, reverse_map: &HashMap<usize, String>) -> OptimizationResult {
    let mut variables = HashMap::new();
    let num_rows = tableau.matrix.rows - 1;
    let rhs_col = tableau.matrix.cols - 1;

    // Las variables básicas toman el valor del RHS en su fila
    for (row_idx, &col_idx) in tableau.basic_vars.iter().enumerate() {
        if row_idx < num_rows {
            let val = tableau.matrix.get(row_idx, rhs_col);
            if let Some(name) = reverse_map.get(&col_idx) {
                // Filtramos variables internas de slack ("_slack_0") si no queremos mostrarlas
                if !name.starts_with("_slack_") {
                    variables.insert(name.clone(), val);
                }
            }
        }
    }

    // El valor objetivo está en la esquina inferior derecha
    // Dependiendo de la convención (Z - ... = 0), puede requerir inversión de signo
    let obj_val = tableau.matrix.get(num_rows, rhs_col);

    Ok(Solution {
        status: OptimizationStatus::Optimal,
        objective_value: obj_val,
        variables,
    })
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::optimization::linear::model::{LinearProblem, Objective, Constraint, LinearExpression, Relation};
    use crate::optimization::linear::algorithms::simplex::solve_primal;
    use crate::optimization::linear::error::{OptimizationStatus, OptimizationError};

    // --- Helpers para reducir verbosidad en los tests ---

    fn expr(terms: &[(&str, f64)], constant: f64) -> LinearExpression {
        let mut e = LinearExpression::new();
        for (name, coeff) in terms {
            e.add_term(name, *coeff);
        }
        e.set_constant(constant);
        e
    }

    // --- Tests ---

    #[test]
    fn test_simple_maximization() {
        // Problema Clásico de Mezcla de Productos
        // Maximizar Z = 3x + 2y
        // Sujeto a:
        //   2x + y <= 100
        //   x + y  <= 80
        //   x      <= 40
        // Solución esperada: x=20, y=60, Z=180

        let objective = Objective::maximize(expr(&[("x", 3.0), ("y", 2.0)], 0.0));
        let mut problem = LinearProblem::new("Test Mix", objective);

        problem.add_constraint(Constraint::new(
            expr(&[("x", 2.0), ("y", 1.0)], 0.0),
            Relation::LessOrEqual,
            100.0
        ).with_name("c1"));

        problem.add_constraint(Constraint::new(
            expr(&[("x", 1.0), ("y", 1.0)], 0.0),
            Relation::LessOrEqual,
            80.0
        ).with_name("c2"));

        problem.add_constraint(Constraint::new(
            expr(&[("x", 1.0)], 0.0),
            Relation::LessOrEqual,
            40.0
        ).with_name("c3"));

        // Ejecutar Solver
        let result = solve_primal(&problem);

        // Aserciones
        assert!(result.is_ok(), "El solver debería encontrar una solución óptima");
        let solution = result.unwrap();

        assert_eq!(solution.status, OptimizationStatus::Optimal);
        
        // Verificamos el valor objetivo (con tolerancia a float)
        assert!((solution.objective_value - 180.0).abs() < 1e-6, 
            "El valor objetivo debería ser 180, se obtuvo {}", solution.objective_value);

        // Verificamos las variables
        let x_val = *solution.variables.get("x").unwrap_or(&0.0);
        let y_val = *solution.variables.get("y").unwrap_or(&0.0);

        assert!((x_val - 20.0).abs() < 1e-6, "x debería ser 20, es {}", x_val);
        assert!((y_val - 60.0).abs() < 1e-6, "y debería ser 60, es {}", y_val);
    }

    #[test]
    fn test_unbounded_problem() {
        // Problema No Acotado
        // Maximizar Z = x
        // Sujeto a: x >= 0 (implicito), y <= 5
        // x no tiene límite superior en la dirección de crecimiento.
        
        let objective = Objective::maximize(expr(&[("x", 1.0)], 0.0));
        let mut problem = LinearProblem::new("Unbounded", objective);

        // Restricción dummy que no limita x
        problem.add_constraint(Constraint::new(
            expr(&[("y", 1.0)], 0.0),
            Relation::LessOrEqual,
            5.0
        ));

        let result = solve_primal(&problem);

        match result {
            Err(OptimizationError::Unbounded) => assert!(true),
            _ => panic!("El problema debería detectarse como NO ACOTADO, resultado: {:?}", result),
        }
    }

    #[test]
    fn test_variable_mapping_consistency() {
        // Verificar que el orden de declaración no afecta el resultado
        // Max Z = 5A + 3B
        // A <= 10
        // B <= 10
        
        let objective = Objective::maximize(expr(&[("B", 3.0), ("A", 5.0)], 0.0)); // Orden mezclado
        let mut problem = LinearProblem::new("Mapping", objective);

        problem.add_constraint(Constraint::new(
            expr(&[("A", 1.0)], 0.0),
            Relation::LessOrEqual,
            10.0
        ));
        
        problem.add_constraint(Constraint::new(
            expr(&[("B", 1.0)], 0.0),
            Relation::LessOrEqual,
            10.0
        ));

        let solution = solve_primal(&problem).expect("Debe tener solución");
        
        assert!((solution.objective_value - 80.0).abs() < 1e-6);
        assert_eq!(*solution.variables.get("A").unwrap(), 10.0);
        assert_eq!(*solution.variables.get("B").unwrap(), 10.0);
    }
}