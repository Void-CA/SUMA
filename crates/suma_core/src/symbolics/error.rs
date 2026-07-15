use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EvalError {
    #[error("Variable not defined: '{0}'")]
    VariableNotFound(String),

    #[error("Division by zero")]
    DivisionByZero,
}

#[cfg(test)]
mod tests {
    use super::EvalError;

    #[test]
    fn test_eval_error_display() {
        let var_error = EvalError::VariableNotFound("x".to_string());
        assert_eq!(format!("{}", var_error), "Variable not defined: 'x'");

        let div_error = EvalError::DivisionByZero;
        assert_eq!(format!("{}", div_error), "Division by zero");
    }
}
