use std::collections::HashSet;
use crate::optimization::linear::model::LinearProblem;

#[derive(Debug, Clone)]
pub struct IntegerProblem {
    pub linear_problem: LinearProblem,
    
    /// Nombres de las variables que deben tomar valores enteros.
    /// Si una variable no está aquí, se asume continua.
    pub integer_variables: HashSet<String>,
}

impl IntegerProblem {
    pub fn new(linear_problem: LinearProblem) -> Self {
        Self {
            linear_problem,
            integer_variables: HashSet::new(),
        }
    }

    /// Marca una variable específica como entera
    pub fn mark_as_integer(&mut self, var_name: &str) {
        self.integer_variables.insert(var_name.to_string());
    }

    /// Marca una lista de variables como enteras
    pub fn mark_many_as_integer(&mut self, vars: &[&str]) {
        for v in vars {
            self.integer_variables.insert(v.to_string());
        }
    }
}