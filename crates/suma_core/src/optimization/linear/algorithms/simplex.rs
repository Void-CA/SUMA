use std::collections::HashMap;
use crate::optimization::linear::model::{LinearProblem, OptimizationDirection};
use crate::optimization::linear::internal::tableau::SimplexTableau;
use crate::optimization::linear::transformers::standard_form::{to_standard_form, StandardFormResult};
use crate::optimization::linear::error::{OptimizationResult, LinearOptimizationError, Solution, OptimizationStatus}; // Nuevo Error

const MAX_ITERATIONS: usize = 10000;
const EPSILON: f64 = 1e-9;

pub fn solve_primal(problem: &LinearProblem) -> OptimizationResult {
    // 1. Convertir modelo
    let StandardFormResult { 
        mut tableau, 
        reverse_map, 
        artificial_indices, 
        original_objective_row, 
        constraint_col_map,
        .. 
    } = to_standard_form(problem)
        .map_err(|e| LinearOptimizationError::ValidationError(format!("{:?}", e)))?;

    // Detectamos la dirección ORIGINAL
    let is_minimization = problem.objective.direction == OptimizationDirection::Minimize;

    let has_artificial_vars = !artificial_indices.is_empty();

    // 2. FASE 1 (Buscar Factibilidad)
    if has_artificial_vars {
        run_simplex_phase(&mut tableau, None)?;
        let w_val = tableau.matrix.get(tableau.matrix.rows - 1, tableau.matrix.cols - 1);
        if w_val.abs() > 1e-5 { // Usar abs() por seguridad
            return Err(LinearOptimizationError::Infeasible);
        }
        prepare_phase_2(&mut tableau, &original_objective_row, &artificial_indices);
    }

    // 3. FASE 2 (Optimizar)
    let ignore_list = if has_artificial_vars { Some(&artificial_indices) } else { None };
    run_simplex_phase(&mut tableau, ignore_list)?;

    // 4. Extraer Resultados
    let mut solution = extract_solution(&tableau, &reverse_map, &constraint_col_map);
    
    // 5. AJUSTE DE SIGNOS (LA CORRECCIÓN)
    // Si el problema original era MAXIMIZAR, invertimos el signo del resultado final.
    // (Porque to_standard_form invirtió la entrada, el solver nos dio -Z).
    if is_minimization {
        // Invertimos valor objetivo (-550 -> 550)
        solution.objective_value = -solution.objective_value;

        // Invertimos Precios Sombra
        for val in solution.shadow_prices.values_mut() {
            *val = -*val;
        }
    }

    Ok(solution)
}

fn run_simplex_phase(
    tableau: &mut SimplexTableau, 
    ignore_cols: Option<&Vec<usize>>
) -> Result<(), LinearOptimizationError> {
    let mut iterations = 0;

    loop {
        if iterations >= MAX_ITERATIONS {
            return Err(LinearOptimizationError::MaxIterationsReached);
        }
        iterations += 1;

        if is_optimal(tableau, ignore_cols) {
            return Ok(());
        }
        
        let pivot_col = match select_entering_variable(tableau, ignore_cols) {
            Some(col) => col,
            None => return Ok(()),
        };

        let pivot_row = match select_leaving_variable(tableau, pivot_col) {
            Some(row) => row,
            None => return Err(LinearOptimizationError::Unbounded),
        };
        
        tableau.pivot(pivot_row, pivot_col);
    }
}

fn prepare_phase_2(
    tableau: &mut SimplexTableau, 
    original_objective: &[f64],
    artificial_indices: &[usize]
) {
    let rows = tableau.matrix.rows;
    let cols = tableau.matrix.cols;
    let z_row_idx = rows - 1;

    // A. Cargar Objetivo
    for col in 0..cols {
        if artificial_indices.contains(&col) {
            tableau.matrix.set(z_row_idx, col, 0.0);
        } else {
            tableau.matrix.set(z_row_idx, col, original_objective[col]);
        }
    }

    // B. Pricing Out
    for (row_idx, &basic_col_idx) in tableau.basic_vars.iter().enumerate() {
        if row_idx < z_row_idx {
            let coeff_in_z = tableau.matrix.get(z_row_idx, basic_col_idx);
            if coeff_in_z.abs() > EPSILON {
                for col in 0..cols {
                    let val_row = tableau.matrix.get(row_idx, col);
                    let val_z = tableau.matrix.get(z_row_idx, col);
                    tableau.matrix.set(z_row_idx, col, val_z - (coeff_in_z * val_row));
                }
            }
        }
    }
}

// --- Helpers ---

fn is_optimal(tableau: &SimplexTableau, ignore_cols: Option<&Vec<usize>>) -> bool {
    let last_row_idx = tableau.matrix.rows - 1;
    for j in 0..(tableau.matrix.cols - 1) { 
        if let Some(ignored) = ignore_cols {
            if ignored.contains(&j) { continue; }
        }
        if tableau.matrix.get(last_row_idx, j) < -EPSILON {
            return false;
        }
    }
    true
}

fn select_entering_variable(tableau: &SimplexTableau, ignore_cols: Option<&Vec<usize>>) -> Option<usize> {
    let last_row_idx = tableau.matrix.rows - 1;
    let mut min_val = -EPSILON;
    let mut entering_col = None;

    for j in 0..(tableau.matrix.cols - 1) {
        if let Some(ignored) = ignore_cols {
            if ignored.contains(&j) { continue; }
        }
        let val = tableau.matrix.get(last_row_idx, j);
        if val < min_val {
            min_val = val;
            entering_col = Some(j);
        }
    }
    entering_col
}

fn select_leaving_variable(tableau: &SimplexTableau, col_idx: usize) -> Option<usize> {
    let mut min_ratio = f64::INFINITY;
    let mut leaving_row = None;

    for i in 0..(tableau.matrix.rows - 1) {
        let coeff = tableau.matrix.get(i, col_idx);
        let rhs = tableau.matrix.get(i, tableau.matrix.cols - 1);

        if coeff > EPSILON {
            let ratio = rhs / coeff;
            if ratio < min_ratio {
                min_ratio = ratio;
                leaving_row = Some(i);
            }
        }
    }
    leaving_row
}

fn extract_solution(
    tableau: &SimplexTableau, 
    reverse_map: &HashMap<usize, String>,
    constraint_col_map: &HashMap<String, usize>
) -> Solution {
    let mut variables = HashMap::new();
    let num_rows = tableau.matrix.rows - 1;
    let rhs_col = tableau.matrix.cols - 1;

    // 1. Variables de Decisión
    for (row_idx, &col_idx) in tableau.basic_vars.iter().enumerate() {
        if row_idx < num_rows {
            let val = tableau.matrix.get(row_idx, rhs_col);
            if let Some(name) = reverse_map.get(&col_idx) {
                if !name.starts_with('_') {
                    variables.insert(name.clone(), val);
                }
            }
        }
    }
    for name in reverse_map.values() {
        if !name.starts_with('_') && !variables.contains_key(name) {
            variables.insert(name.clone(), 0.0);
        }
    }

    // 2. Shadow Prices (Precios Sombra)
    let mut shadow_prices = HashMap::new();
    for (name, &col_idx) in constraint_col_map {
        let val = tableau.matrix.get(num_rows, col_idx);
        shadow_prices.insert(name.clone(), val);
    }

    let obj_val = tableau.matrix.get(num_rows, rhs_col);

    Solution {
        status: OptimizationStatus::Optimal,
        objective_value: obj_val,
        variables,
        shadow_prices,
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::optimization::linear::model::{LinearProblem, Objective, Constraint, LinearExpression, Relation};
    use crate::optimization::linear::algorithms::simplex::solve_primal;
    use crate::optimization::linear::error::{OptimizationStatus, LinearOptimizationError};

    fn expr(terms: &[(&str, f64)], constant: f64) -> LinearExpression {
        let mut e = LinearExpression::new();
        for (name, coeff) in terms { e.add_term(name, *coeff); }
        e.set_constant(constant);
        e
    }

    #[test]
    fn test_simple_maximization() {
        let objective = Objective::maximize(expr(&[("x", 3.0), ("y", 2.0)], 0.0));
        let mut problem = LinearProblem::new("Test Mix", objective);
        problem.add_constraint(Constraint::new(expr(&[("x", 2.0), ("y", 1.0)], 0.0), Relation::LessOrEqual, 100.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0), ("y", 1.0)], 0.0), Relation::LessOrEqual, 80.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::LessOrEqual, 40.0));
        let solution = solve_primal(&problem).unwrap();
        assert!((solution.objective_value - 180.0).abs() < 1e-6);
    }

    #[test]
fn test_sensitivity_analysis() {
    // Max Z = 30x + 50y. 
    // Madera: x + 2y <= 20
    // Horas:  x <= 10
    let objective = Objective::maximize(expr(&[("x", 30.0), ("y", 50.0)], 0.0));
    let mut problem = LinearProblem::new("Sensitivity", objective);

    problem.add_constraint(Constraint::new(expr(&[("x", 1.0), ("y", 2.0)], 0.0), Relation::LessOrEqual, 20.0).with_name("Madera"));
    problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::LessOrEqual, 10.0).with_name("Horas"));
    
    let solution = solve_primal(&problem).unwrap();
    assert!((solution.objective_value - 550.0).abs() < 1e-6);

    // Validar Precios Sombra
    let shadow_madera = *solution.shadow_prices.get("Madera").unwrap();
    let shadow_horas = *solution.shadow_prices.get("Horas").unwrap();
    
    // Madera: 25.0
    assert!((shadow_madera - 25.0).abs() < 1e-6, "Shadow Madera: {}", shadow_madera);
    // Horas: 5.0
    assert!((shadow_horas - 5.0).abs() < 1e-6, "Shadow Horas: {}", shadow_horas);
}

#[test]
fn test_two_phase_minimization() {
    let objective = Objective::minimize(expr(&[("x", 2.0), ("y", 3.0)], 0.0));
    let mut problem = LinearProblem::new("Phase 1 Min", objective);
    problem.add_constraint(Constraint::new(expr(&[("x", 1.0), ("y", 1.0)], 0.0), Relation::GreaterOrEqual, 10.0));
    problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::GreaterOrEqual, 2.0));
    
    let solution = solve_primal(&problem).expect("Solución Factible");
    assert_eq!(solution.status, OptimizationStatus::Optimal);
    assert!((solution.objective_value - 20.0).abs() < 1e-6);
}

    #[test]
    fn test_infeasible_problem() {
        let objective = Objective::maximize(expr(&[("x", 1.0)], 0.0));
        let mut problem = LinearProblem::new("Infeasible", objective);
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::LessOrEqual, 5.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::GreaterOrEqual, 10.0));
        match solve_primal(&problem) {
            Err(LinearOptimizationError::Infeasible) => assert!(true),
            _ => panic!("Expected Infeasible"),
        }
    }

    #[test]
    fn test_equality_constraint() {
        let objective = Objective::maximize(expr(&[("x", 1.0), ("y", 1.0)], 0.0));
        let mut problem = LinearProblem::new("Equality", objective);
        problem.add_constraint(Constraint::new(expr(&[("x", 2.0), ("y", 1.0)], 0.0), Relation::Equal, 10.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::LessOrEqual, 3.0));
        let solution = solve_primal(&problem).unwrap();
        assert!((solution.objective_value - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_unbounded_problem() {
        let objective = Objective::maximize(expr(&[("x", 1.0)], 0.0));
        let mut problem = LinearProblem::new("Unbounded", objective);
        problem.add_constraint(Constraint::new(expr(&[("y", 1.0)], 0.0), Relation::LessOrEqual, 5.0));
        match solve_primal(&problem) {
            Err(LinearOptimizationError::Unbounded) => assert!(true),
            _ => panic!("Expected Unbounded"),
        }
    }
    
    #[test]
    fn test_simple_minimization() {
        let objective = Objective::minimize(expr(&[("x", -3.0), ("y", -2.0)], 0.0));
        let mut problem = LinearProblem::new("Test Min", objective);
        problem.add_constraint(Constraint::new(expr(&[("x", 2.0), ("y", 1.0)], 0.0), Relation::LessOrEqual, 100.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0), ("y", 1.0)], 0.0), Relation::LessOrEqual, 80.0));
        problem.add_constraint(Constraint::new(expr(&[("x", 1.0)], 0.0), Relation::LessOrEqual, 40.0));
        let solution = solve_primal(&problem).unwrap();
        assert!((solution.objective_value - (-180.0)).abs() < 1e-6);
    }

    #[test]
    fn test_variable_mapping_consistency() {
        let objective = Objective::maximize(expr(&[("B", 3.0), ("A", 5.0)], 0.0)); 
        let mut problem = LinearProblem::new("Mapping", objective);
        problem.add_constraint(Constraint::new(expr(&[("A", 1.0)], 0.0), Relation::LessOrEqual, 10.0));
        problem.add_constraint(Constraint::new(expr(&[("B", 1.0)], 0.0), Relation::LessOrEqual, 10.0));
        let solution = solve_primal(&problem).expect("Debe tener solución");
        assert!((solution.objective_value - 80.0).abs() < 1e-6);
    }
}