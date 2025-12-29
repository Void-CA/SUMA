use std::collections::VecDeque;
use crate::optimization::linear::algorithms::simplex::solve_primal;
use crate::optimization::linear::model::{Constraint, LinearExpression, Relation, OptimizationDirection};
use crate::optimization::linear::error::{OptimizationStatus, Solution};
use crate::optimization::error::OptimizationError; // Usamos el error genérico
use crate::optimization::integer::problem::IntegerProblem;

const EPSILON: f64 = 1e-6;

/// Resuelve un problema de Programación Entera Mixta (MILP) usando Branch & Bound.
pub fn solve_integer(problem: &IntegerProblem) -> Result<Solution, OptimizationError> {
    let direction = problem.linear_problem.objective.direction;
    let is_minimization = direction == OptimizationDirection::Minimize;

    // Mejor solución entera encontrada hasta el momento (Incumbent)
    let mut best_solution: Option<Solution> = None;
    let mut best_obj_value = if is_minimization { f64::INFINITY } else { f64::NEG_INFINITY };

    // Cola de problemas por explorar (Nodes)
    // Usamos VecDeque como Stack para DFS (Depth First Search) para encontrar soluciones rápido
    let mut queue = VecDeque::new();
    queue.push_back(problem.linear_problem.clone());

    let mut iterations = 0;
    let max_nodes = 1000; // Seguridad para evitar loops infinitos en problemas grandes

    while let Some(current_prob) = queue.pop_back() {
        iterations += 1;
        if iterations > max_nodes {
            break; // O retornar error de límite
        }

        // 1. Resolver Relajación Lineal (Simplex)
        let result = solve_primal(&current_prob);

        match result {
            Ok(sol) => {
                // 2. Poda por Acotamiento (Bound)
                // Si la solución relajada es peor que la mejor entera que ya tenemos,
                // no vale la pena seguir explorando esta rama.
                if is_minimization {
                    if sol.objective_value >= best_obj_value { continue; }
                } else {
                    if sol.objective_value <= best_obj_value { continue; }
                }

                // 3. Chequeo de Integridad
                // Buscamos la primera variable que DEBERÍA ser entera pero no lo es.
                if let Some((var_name, val)) = find_fractional_var(&sol, &problem.integer_variables) {
                    // --- RAMIFICACIÓN (BRANCH) ---
                    // La variable es fraccional (ej. 3.4). Creamos dos ramas.
                    
                    let floor_val = val.floor();
                    let ceil_val = val.ceil();

                    // Rama 1: var <= floor (ej. x <= 3)
                    let mut left_prob = current_prob.clone();
                    left_prob.add_constraint(Constraint::new(
                        var_to_expr(&var_name),
                        Relation::LessOrEqual,
                        floor_val
                    ));
                    queue.push_back(left_prob);

                    // Rama 2: var >= ceil (ej. x >= 4)
                    let mut right_prob = current_prob.clone();
                    right_prob.add_constraint(Constraint::new(
                        var_to_expr(&var_name),
                        Relation::GreaterOrEqual,
                        ceil_val
                    ));
                    queue.push_back(right_prob);

                } else {
                    // --- SOLUCIÓN ENTERA ENCONTRADA ---
                    // Todas las variables requeridas son enteras.
                    // Como pasamos el filtro de Bound, esta solución es MEJOR que la anterior.
                    best_obj_value = sol.objective_value;
                    best_solution = Some(sol);
                }
            },
            Err(_) => {
                // Infactible o error numérico en esta rama. Podar.
                continue;
            }
        }
    }

    match best_solution {
        Some(sol) => Ok(sol),
        None => Err(crate::optimization::linear::error::LinearOptimizationError::Infeasible.into()),
    }
}

// --- Helpers ---

/// Busca una variable que está marcada como entera pero tiene valor decimal.
fn find_fractional_var(sol: &Solution, int_vars: &std::collections::HashSet<String>) -> Option<(String, f64)> {
    for (name, &val) in &sol.variables {
        if int_vars.contains(name) {
            // Chequeamos si es fraccional: abs(val - round(val)) > epsilon
            if (val - val.round()).abs() > EPSILON {
                return Some((name.clone(), val));
            }
        }
    }
    None
}

/// Helper para crear una expresión lineal simple "1 * x"
fn var_to_expr(name: &str) -> LinearExpression {
    let mut expr = LinearExpression::new();
    expr.add_term(name, 1.0);
    expr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::linear::model::{LinearProblem, Objective, Constraint, LinearExpression, Relation};
    use crate::optimization::integer::branch_bound::IntegerProblem;

    // Helper para expresiones (Reciclado de linear)
    fn expr(terms: &[(&str, f64)], constant: f64) -> LinearExpression {
        let mut e = LinearExpression::new();
        for (name, coeff) in terms {
            e.add_term(name, *coeff);
        }
        e.set_constant(constant);
        e
    }

    #[test]
    fn test_pure_integer_rounding_gap() {
        // Problema Clásico: La solución LP no es entera y redondear viola restricciones.
        // Maximizar Z = x + y
        // Sujeto a: 2x + 2y <= 9
        //
        // Solución LP (Simplex):
        // 2(x+y) <= 9 -> x+y <= 4.5.
        // Óptimo LP: x=2.25, y=2.25 (o cualquier combinación que sume 4.5). Z = 4.5.
        //
        // Solución Entera (B&B):
        // x, y enteros. x+y debe ser entero.
        // Max entero <= 4.5 es 4.
        // Óptimo IP: x=2, y=2 (Z=4).
        
        let objective = Objective::maximize(expr(&[("x", 1.0), ("y", 1.0)], 0.0));
        let mut linear = LinearProblem::new("RoundingGap", objective);
        
        linear.add_constraint(Constraint::new(
            expr(&[("x", 2.0), ("y", 2.0)], 0.0), 
            Relation::LessOrEqual, 
            9.0
        ));

        // Marcamos AMBAS como enteras
        let mut problem = IntegerProblem::new(linear);
        problem.mark_many_as_integer(&["x", "y"]);

        let solution = solve_integer(&problem).expect("Debe tener solución entera");

        // Verificamos que el valor objetivo bajó de 4.5 a 4.0
        assert!((solution.objective_value - 4.0).abs() < 1e-6, 
            "El valor objetivo entero debería ser 4.0, se obtuvo {}", solution.objective_value);
        
        // Verificamos integridad de las variables
        let x = *solution.variables.get("x").unwrap();
        let y = *solution.variables.get("y").unwrap();
        
        assert!((x - x.round()).abs() < 1e-6, "x debe ser entero");
        assert!((y - y.round()).abs() < 1e-6, "y debe ser entero");
    }

    #[test]
    fn test_integer_infeasible() {
        // Problema: x debe ser 0.5
        // Max x. Sujeto a x = 0.5
        // Si x es entero, esto es imposible.
        
        let objective = Objective::maximize(expr(&[("x", 1.0)], 0.0));
        let mut linear = LinearProblem::new("IntInfeasible", objective);
        
        linear.add_constraint(Constraint::new(
            expr(&[("x", 1.0)], 0.0), 
            Relation::Equal, 
            0.5
        ));

        let mut problem = IntegerProblem::new(linear);
        problem.mark_as_integer("x");

        let result = solve_integer(&problem);

        match result {
            Err(OptimizationError::Linear(crate::optimization::linear::error::LinearOptimizationError::Infeasible)) => assert!(true),
            _ => panic!("Debería ser infactible en enteros, resultado: {:?}", result),
        }
    }

    #[test]
    fn test_mixed_integer_programming() {
        // Problema MIP: x entero, y continuo.
        // Max Z = x + y
        // Sujeto a: 2x + 2y <= 9
        //
        // Solución LP: x=2.25, y=2.25, Z=4.5
        //
        // Solución MIP:
        // x debe ser entero. Max x posible en 2x <= 9 es x=4 (si y=0.5).
        // Si x=4: 8 + 2y <= 9 -> 2y <= 1 -> y <= 0.5.
        // Z = 4 + 0.5 = 4.5.
        //
        // Esperamos que el solver encuentre Z=4.5, pero con x entero (4.0) y y decimal (0.5).
        // Ojo: x=0, y=4.5 también es válido y da 4.5.
        
        let objective = Objective::maximize(expr(&[("x", 1.0), ("y", 1.0)], 0.0));
        let mut linear = LinearProblem::new("MIP", objective);
        
        linear.add_constraint(Constraint::new(
            expr(&[("x", 2.0), ("y", 2.0)], 0.0), 
            Relation::LessOrEqual, 
            9.0
        ));

        let mut problem = IntegerProblem::new(linear);
        problem.mark_as_integer("x"); // Solo x es entero

        let solution = solve_integer(&problem).expect("Solución MIP factible");

        // El valor objetivo sigue siendo 4.5 porque 'y' absorbe la holgura
        assert!((solution.objective_value - 4.5).abs() < 1e-6);

        let x = *solution.variables.get("x").unwrap();
        let y = *solution.variables.get("y").unwrap();

        // x debe ser entero
        assert!((x - x.round()).abs() < 1e-6, "x debe ser entero (obtenido {})", x);
        
        // Verificamos que la solución cumple la restricción
        assert!(2.0*x + 2.0*y <= 9.0 + 1e-6);
    }
}