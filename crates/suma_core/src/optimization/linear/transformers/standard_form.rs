use std::collections::{HashMap, HashSet};
use crate::optimization::linear::model::{LinearProblem, Relation, OptimizationDirection};
use crate::optimization::linear::internal::tableau::SimplexTableau;
use crate::optimization::linear::error::LinearOptimizationError; // Cambio de nombre
use crate::zeros;

pub struct StandardFormResult {
    pub tableau: SimplexTableau,
    pub var_map: HashMap<String, usize>,
    pub reverse_map: HashMap<usize, String>,
    pub artificial_indices: Vec<usize>,
    pub original_objective_row: Vec<f64>,
    
    // Mapa para rastrear qué columna corresponde a la holgura de qué restricción
    pub constraint_col_map: HashMap<String, usize>,
}

pub fn to_standard_form(problem: &LinearProblem) -> Result<StandardFormResult, LinearOptimizationError> {
    // 1. Recolectar variables
    let mut vars: Vec<String> = problem.get_variables().into_iter().collect();
    vars.sort(); 
    let num_decision_vars = vars.len();
    let num_constraints = problem.constraints.len();

    // 2. Contar variables auxiliares
    let mut num_slack = 0;
    let mut num_artificial = 0;

    for c in &problem.constraints {
        match c.relation {
            Relation::LessOrEqual => num_slack += 1,
            Relation::GreaterOrEqual => {
                num_slack += 1;
                num_artificial += 1;
            },
            Relation::Equal => num_artificial += 1,
        }
    }

    let num_total_vars = num_decision_vars + num_slack + num_artificial;
    let rows = num_constraints + 1;
    let cols = num_total_vars + 1;

    // 3. Crear Mapas
    let mut var_map = HashMap::new();
    let mut reverse_map = HashMap::new();
    for (i, name) in vars.iter().enumerate() {
        var_map.insert(name.clone(), i);
        reverse_map.insert(i, name.clone());
    }

    // 4. Inicializar Matriz
    let mut matrix = zeros!(rows, cols);
    let mut basic_vars = vec![0; num_constraints];
    let mut artificial_indices = Vec::new();
    let mut constraint_col_map = HashMap::new(); // Nuevo mapa

    let mut current_slack_col = num_decision_vars;
    let mut current_artificial_col = num_decision_vars + num_slack;

    // 5. Llenar Restricciones
    for (row_idx, constraint) in problem.constraints.iter().enumerate() {
        // A) Coeficientes decisión
        for (var_name, coeff) in &constraint.lhs.coefficients {
            if let Some(&col_idx) = var_map.get(var_name) {
                matrix.set(row_idx, col_idx, *coeff);
            }
        }

        // B) Vars Auxiliares y Mapeo para Shadow Prices
        match constraint.relation {
            Relation::LessOrEqual => {
                matrix.set(row_idx, current_slack_col, 1.0);
                reverse_map.insert(current_slack_col, format!("_s_{}", row_idx));
                basic_vars[row_idx] = current_slack_col;
                
                if let Some(name) = &constraint.name {
                    constraint_col_map.insert(name.clone(), current_slack_col);
                }
                
                current_slack_col += 1;
            },
            Relation::GreaterOrEqual => {
                matrix.set(row_idx, current_slack_col, -1.0);
                reverse_map.insert(current_slack_col, format!("_surplus_{}", row_idx));
                
                // En >=, el shadow price se lee del surplus
                if let Some(name) = &constraint.name {
                    constraint_col_map.insert(name.clone(), current_slack_col);
                }
                current_slack_col += 1;

                matrix.set(row_idx, current_artificial_col, 1.0);
                reverse_map.insert(current_artificial_col, format!("_art_{}", row_idx));
                artificial_indices.push(current_artificial_col);
                basic_vars[row_idx] = current_artificial_col;
                current_artificial_col += 1;
            },
            Relation::Equal => {
                matrix.set(row_idx, current_artificial_col, 1.0);
                reverse_map.insert(current_artificial_col, format!("_art_{}", row_idx));
                artificial_indices.push(current_artificial_col);
                basic_vars[row_idx] = current_artificial_col;
                current_artificial_col += 1;
            }
        }
        // C) RHS
        matrix.set(row_idx, cols - 1, constraint.rhs);
    }

    // 6. Construir Funciones Objetivo
    let mut original_objective_row = vec![0.0; cols]; 
    let is_minimization = problem.objective.direction == OptimizationDirection::Minimize;

    for (var_name, coeff) in &problem.objective.expression.coefficients {
        if let Some(&col_idx) = var_map.get(var_name) {
            let val = if is_minimization { *coeff } else { -*coeff };
            original_objective_row[col_idx] = val;
        }
    }

    // Configurar fila Z
    let z_row_idx = rows - 1;

    if !artificial_indices.is_empty() {
        // FASE 1
        for &art_col in &artificial_indices {
            matrix.set(z_row_idx, art_col, 1.0);
        }
        for row_idx in 0..num_constraints {
            if artificial_indices.contains(&basic_vars[row_idx]) {
                for col in 0..cols {
                    let val_row = matrix.get(row_idx, col);
                    let current_z = matrix.get(z_row_idx, col);
                    matrix.set(z_row_idx, col, current_z - val_row);
                }
            }
        }
    } else {
        // NO FASE 1
        for (col, &val) in original_objective_row.iter().enumerate() {
            matrix.set(z_row_idx, col, val);
        }
    }

    let non_basic_vars: Vec<usize> = (0..num_total_vars)
        .filter(|v| !basic_vars.contains(v))
        .collect();

    Ok(StandardFormResult {
        tableau: SimplexTableau { matrix, basic_vars, non_basic_vars },
        var_map,
        reverse_map,
        artificial_indices,
        original_objective_row,
        constraint_col_map,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::linear::model::{LinearProblem, Objective, Constraint, LinearExpression, Relation, OptimizationDirection};

    // Helper para crear expresiones rápidas
    fn quick_expr(terms: Vec<(&str, f64)>) -> LinearExpression {
        let mut e = LinearExpression::new();
        for (k, v) in terms { e.add_term(k, v); }
        e
    }

    #[test]
    fn test_maximization_coefficients_are_inverted() {
        // Caso: Maximizar 3x + 5y
        // Esperamos que en el Tableau (diseñado para minimizar) aparezcan como -3 y -5
        
        let objective = Objective {
            direction: OptimizationDirection::Maximize,
            expression: quick_expr(vec![("x", 3.0), ("y", 5.0)]),
        };
        let problem = LinearProblem::new("MaxTest", objective);
        
        // Convertimos
        let res = to_standard_form(&problem).expect("Fallo standard form");
        
        // Verificamos la fila Z (última fila si no hay artificiales, o original_objective_row)
        let x_idx = *res.var_map.get("x").unwrap();
        let y_idx = *res.var_map.get("y").unwrap();

        // Verificamos original_objective_row (usado para Fase 2)
        assert_eq!(res.original_objective_row[x_idx], -3.0, "Coeficiente X debe invertirse en Max");
        assert_eq!(res.original_objective_row[y_idx], -5.0, "Coeficiente Y debe invertirse en Max");

        // Verificamos el Tableau directo (asumiendo que no hay artificiales, pasa directo)
        let z_row = res.tableau.matrix.rows - 1;
        assert_eq!(res.tableau.matrix.get(z_row, x_idx), -3.0);
    }

    #[test]
    fn test_minimization_coefficients_are_preserved() {
        // Caso: Minimizar 3x + 5y
        // Esperamos 3 y 5 positivos
        
        let objective = Objective {
            direction: OptimizationDirection::Minimize,
            expression: quick_expr(vec![("x", 3.0), ("y", 5.0)]),
        };
        let problem = LinearProblem::new("MinTest", objective);
        
        let res = to_standard_form(&problem).expect("Fallo standard form");
        
        let x_idx = *res.var_map.get("x").unwrap();
        
        assert_eq!(res.original_objective_row[x_idx], 3.0, "Coeficiente X debe mantenerse en Min");
    }

    #[test]
    fn test_constraint_slack_mapping() {
        // x <= 10  (Slack)
        // x >= 5   (Surplus + Artificial)
        let objective = Objective::maximize(quick_expr(vec![("x", 1.0)]));
        let mut problem = LinearProblem::new("ConstrTest", objective);
        
        problem.add_constraint(Constraint::new(quick_expr(vec![("x", 1.0)]), Relation::LessOrEqual, 10.0)); // Fila 0
        problem.add_constraint(Constraint::new(quick_expr(vec![("x", 1.0)]), Relation::GreaterOrEqual, 5.0)); // Fila 1

        let res = to_standard_form(&problem).unwrap();
        let matrix = &res.tableau.matrix;

        // Fila 0: Slack normal (+1)
        // El nombre interno debería ser _s_0
        let s0_idx = res.reverse_map.iter().find(|(_, v)| *v == "_s_0").map(|(k, _)| *k).unwrap();
        assert_eq!(matrix.get(0, s0_idx), 1.0);

        // Fila 1: Surplus (-1) y Artificial (+1)
        let sur1_idx = res.reverse_map.iter().find(|(_, v)| *v == "_surplus_1").map(|(k, _)| *k).unwrap();
        let art1_idx = res.reverse_map.iter().find(|(_, v)| *v == "_art_1").map(|(k, _)| *k).unwrap();
        
        assert_eq!(matrix.get(1, sur1_idx), -1.0);
        assert_eq!(matrix.get(1, art1_idx), 1.0);
    }
}