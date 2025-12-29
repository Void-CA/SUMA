// src/core/optimization/linear/transformers/standard_form.rs

use std::collections::{HashMap, HashSet};
use crate::optimization::linear::model::{LinearProblem, Relation};
use crate::optimization::linear::internal::tableau::SimplexTableau;
use crate::optimization::linear::error::OptimizationError;
// Asumimos que LinearAlgebra expone constructores de matrices
use crate::zeros; 

/// Resultado de la transformación: El Tableau listo para iterar y el mapa para descifrar la respuesta.
pub struct StandardFormResult {
    pub tableau: SimplexTableau,
    pub var_map: HashMap<String, usize>,
    pub reverse_map: HashMap<usize, String>, // Para reportar la solución final
}

pub fn to_standard_form(problem: &LinearProblem) -> Result<StandardFormResult, OptimizationError> {
    // 1. Recolectar todas las variables únicas (decisión)
    let mut vars: Vec<String> = problem.get_variables().into_iter().collect();
    vars.sort(); // Ordenar para determinismo

    let num_decision_vars = vars.len();
    let num_constraints = problem.constraints.len();
    
    // El número total de columnas será: Vars Decisión + Vars Holgura/Exceso
    // (Simplificación: Asumimos 1 variable de holgura por restricción para empezar)
    let num_total_vars = num_decision_vars + num_constraints;

    // 2. Crear Mapas de Índices
    let mut var_map = HashMap::new();
    let mut reverse_map = HashMap::new();

    for (i, name) in vars.iter().enumerate() {
        var_map.insert(name.clone(), i);
        reverse_map.insert(i, name.clone());
    }

    // Nombrar variables de holgura/exceso internamente
    for i in 0..num_constraints {
        let slack_idx = num_decision_vars + i;
        let slack_name = format!("_slack_{}", i);
        reverse_map.insert(slack_idx, slack_name);
        // No necesitamos agregarlas al var_map de entrada del usuario necesariamente, 
        // pero sí al reverso para debug.
    }

    // 3. Inicializar Matriz (M filas x N columnas + 1 RHS)
    // Usamos el módulo de matrices de SUMA
    // Dimensiones: filas = num_constraints + 1 (función objetivo), cols = num_total_vars + 1 (RHS)
    let rows = num_constraints + 1;
    let cols = num_total_vars + 1;
    
    let mut matrix = zeros!(rows, cols); 

    // 4. Llenar Restricciones (Filas 0 a N-1)
    for (row_idx, constraint) in problem.constraints.iter().enumerate() {
        // A) Coeficientes de variables de decisión
        for (var_name, coeff) in &constraint.lhs.coefficients {
            if let Some(&col_idx) = var_map.get(var_name) {
                matrix.set(row_idx, col_idx, *coeff);
            }
        }

        // B) Variable de Holgura / Exceso
        // Columna de holgura correspondiente a esta fila
        let slack_col = num_decision_vars + row_idx; 
        
        match constraint.relation {
            Relation::LessOrEqual => {
                // lhs + s = rhs  ->  coeff de s es +1
                matrix.set(row_idx, slack_col, 1.0);
            },
            Relation::GreaterOrEqual => {
                // lhs - e = rhs  ->  coeff de e es -1
                matrix.set(row_idx, slack_col, -1.0);
                // Nota: Esto requerirá Fase 1 o Método de la M Grande (Big M) 
                // para encontrar una base factible inicial.
                // Por ahora, asumimos origen factible o manejamos LessOrEqual.
            },
            Relation::Equal => {
                // Requiere variable artificial. Fuera del alcance inicial de este snippet,
                // pero aquí iría la lógica.
            }
        }

        // C) Lado Derecho (RHS)
        // La última columna es el RHS
        // Nota: Si rhs < 0, deberíamos multiplicar toda la fila por -1 primero.
        matrix.set(row_idx, cols - 1, constraint.rhs);
    }

    // 5. Llenar Función Objetivo (Última fila)
    // En el Tableau estándar (Maximización): Z - c1x1 - c2x2 = 0
    // Por tanto, los coeficientes van con signo invertido.
    let obj_row_idx = rows - 1;
    for (var_name, coeff) in &problem.objective.expression.coefficients {
        if let Some(&col_idx) = var_map.get(var_name) {
            // Invertimos signo para formato Z-Row
            matrix.set(obj_row_idx, col_idx, -coeff); 
        }
    }
    
    // Las variables básicas iniciales son las holguras (si existen)
    let basic_vars = (0..num_constraints).map(|i| num_decision_vars + i).collect();
    let non_basic_vars = (0..num_decision_vars).collect();

    Ok(StandardFormResult {
        tableau: SimplexTableau {
            matrix,
            basic_vars,
            non_basic_vars,
        },
        var_map,
        reverse_map,
    })
}