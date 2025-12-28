use crate::ast::CodexResult;
use std::error::Error;
use std::any::Any;

pub type DomainResult = Result<Box<dyn Any>, Box<dyn Error>>;

pub trait DomainParser {
    // CAMBIO: Ahora el parser define una LISTA de palabras clave que acepta
    fn valid_keywords(&self) -> Vec<&'static str>;
    
    fn parse_domain(&self, content: &str) -> DomainResult;
}