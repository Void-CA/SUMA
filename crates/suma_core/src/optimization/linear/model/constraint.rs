use std::fmt;
use super::LinearExpression;

/// Define la relación lógica entre el lado izquierdo (LHS) y el derecho (RHS).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    LessOrEqual,    // <=
    GreaterOrEqual, // >=
    Equal,          // =
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Relation::LessOrEqual => write!(f, "<="),
            Relation::GreaterOrEqual => write!(f, ">="),
            Relation::Equal => write!(f, "="),
        }
    }
}

/// Representa una restricción lineal: LHS [Relación] RHS
/// Ejemplo: 2x + y <= 10
#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    /// Nombre opcional para identificar la restricción (ej: "Limite_Horas")
    pub name: Option<String>,
    
    /// La expresión lineal (las variables y coeficientes)
    pub lhs: LinearExpression,
    
    /// El tipo de comparación (<=, >=, =)
    pub relation: Relation,
    
    /// El valor constante del lado derecho.
    /// Nota: En LP estándar, las variables van a la izquierda y las constantes a la derecha.
    pub rhs: f64,
}

impl Constraint {
    /// Crea una nueva restricción sin nombre
    pub fn new(lhs: LinearExpression, relation: Relation, rhs: f64) -> Self {
        Self {
            name: None,
            lhs,
            relation,
            rhs,
        }
    }

    /// Asigna un nombre a la restricción (patrón Builder)
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Helper para verificar si una solución dada cumple esta restricción
    /// (Útil para validar resultados finales o puntos manuales)
    pub fn is_satisfied(&self, var_values: &std::collections::HashMap<String, f64>) -> bool {
        // Evaluamos el LHS con los valores dados
        // Nota: Asumimos que LinearExpression tiene un método `evaluate`.
        // Si no lo tiene, deberíamos agregarlo en su implementación.
        let val = self.lhs.evaluate(var_values);
        
        // Usamos un epsilon pequeño para evitar errores de punto flotante
        let epsilon = 1e-9;

        match self.relation {
            Relation::LessOrEqual => val <= self.rhs + epsilon,
            Relation::GreaterOrEqual => val >= self.rhs - epsilon,
            Relation::Equal => (val - self.rhs).abs() < epsilon,
        }
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(n) = &self.name {
            write!(f, "[{}] ", n)?;
        }
        write!(f, "{} {} {}", self.lhs, self.relation, self.rhs)
    }
}