use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Clone)] // Agregamos PartialEq y Clone para facilitar tests
pub enum EvalError {
    VariableNotFound(String),
    DivisionByZero,
    // Futuro: Podríamos agregar errores de sintaxis o tipos incompatibles aquí
}

// Implementación de Display para errores (buena práctica para el CLI/REPL)
impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::VariableNotFound(v) => write!(f, "Variable no definida: '{}'", v),
            EvalError::DivisionByZero => write!(f, "División por cero detectada"),
        }
    }
}

// 2. Implementación de std::error::Error (Lo que usa el sistema)
impl Error for EvalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // Como estos son errores "hoja" (no envuelven otros errores como IO o Parse),
            // devolvemos None. Si tuviéramos un error que envuelve otro, lo devolveríamos aquí.
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EvalError;
    
    #[test]
    fn test_eval_error_display() {
        let var_error = EvalError::VariableNotFound("x".to_string());
        assert_eq!(format!("{}", var_error), "Variable no definida: 'x'");
        
        let div_error = EvalError::DivisionByZero;
        assert_eq!(format!("{}", div_error), "División por cero detectada");
    }
}