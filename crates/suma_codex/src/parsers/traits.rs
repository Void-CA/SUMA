use crate::ast::CodexResult;
use crate::error::CodexError;

pub type DomainResult = Result<CodexResult, CodexError>;

pub trait DomainParser {
    // CAMBIO: Ahora el parser define una LISTA de palabras clave que acepta
    fn valid_keywords(&self) -> Vec<&'static str>;
    
    fn parse_domain(&self, content: &str) -> DomainResult;
}