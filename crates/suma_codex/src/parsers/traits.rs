// src/parsers/traits.rs
use std::any::Any;

// Usamos Box<dyn Any> porque cada dominio devuelve un AST totalmente diferente.
pub type DomainResult = Result<Box<dyn Any>, String>;

pub trait DomainParser {
    // Cada parser debe saber procesar un string y devolver "algo"
    fn parse_domain(&self, content: &str) -> DomainResult;
    
    // Metadatos útiles
    fn domain_name(&self) -> &'static str;
}