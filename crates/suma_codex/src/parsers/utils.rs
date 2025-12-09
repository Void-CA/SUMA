// src/parsers/utils.rs
use pest::error::Error;
use pest::RuleType;

// Función genérica que convierte errores de PEST a Strings legibles
pub fn format_pest_error<R: RuleType>(e: Error<R>) -> String {
    format!("Error de sintaxis en dominio: {}", e)
}
