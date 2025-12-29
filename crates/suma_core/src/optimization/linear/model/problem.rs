use std::collections::HashSet;
use super::{Objective, Constraint};

/// Estructura principal que agrupa todo el modelo de optimización lineal.
#[derive(Debug, Clone)]
pub struct LinearProblem {
    pub name: String,
    pub objective: Objective,
    pub constraints: Vec<Constraint>,
}

impl LinearProblem {
    /// Crea un nuevo problema lineal con un nombre y una función objetivo.
    pub fn new(name: &str, objective: Objective) -> Self {
        Self {
            name: name.to_string(),
            objective,
            constraints: Vec::new(),
        }
    }

    /// Agrega una restricción al problema.
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    /// Recolecta todos los nombres de variables únicos usados en el problema.
    /// Esto es vital para saber cuántas columnas (N) tendrá nuestra matriz.
    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars = HashSet::new();

        // 1. Buscar variables en la función objetivo
        for key in self.objective.expression.coefficients.keys() {
            vars.insert(key.clone());
        }

        // 2. Buscar variables en cada restricción
        for c in &self.constraints {
            for key in c.lhs.coefficients.keys() {
                vars.insert(key.clone());
            }
        }

        vars
    }
}