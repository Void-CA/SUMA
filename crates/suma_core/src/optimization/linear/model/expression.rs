// En src/core/optimization/linear/model/mod.rs

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearExpression {
    pub coefficients: HashMap<String, f64>,
    pub constant: f64,
}

impl LinearExpression {
    pub fn new() -> Self {
        Self { coefficients: HashMap::new(), constant: 0.0 }
    }
    
    pub fn add_term(&mut self, var: &str, coeff: f64) {
        *self.coefficients.entry(var.to_string()).or_insert(0.0) += coeff;
    }
    
    pub fn set_constant(&mut self, constant: f64) {
        self.constant = constant;
    }
    // Método necesario para `Constraint::is_satisfied`
    pub fn evaluate(&self, vars: &HashMap<String, f64>) -> f64 {
        let mut sum = self.constant;
        for (var_name, coeff) in &self.coefficients {
            let val = vars.get(var_name).cloned().unwrap_or(0.0);
            sum += coeff * val;
        }
        sum
    }
}

// Display para que se vea como: "3*x + 2*y"
impl fmt::Display for LinearExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut terms: Vec<_> = self.coefficients.iter().collect();
        // Ordenamos por nombre de variable para consistencia visual
        terms.sort_by_key(|k| k.0);

        if terms.is_empty() {
            return write!(f, "{}", self.constant);
        }

        for (i, (var, coeff)) in terms.iter().enumerate() {
            if i > 0 && *coeff >= &0.0 {
                write!(f, " + ")?;
            } else if i > 0 {
                write!(f, " - ")?; // Manejo simplificado del signo
            }
            let val = if i > 0 { coeff.abs() } else { **coeff };
            write!(f, "{}*{}", val, var)?;
        }
        
        if self.constant != 0.0 {
            write!(f, " + {}", self.constant)?;
        }
        Ok(())
    }
}