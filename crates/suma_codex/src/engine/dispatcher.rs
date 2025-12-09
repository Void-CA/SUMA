use std::collections::HashMap;
use std::any::Any;

// Imports internos
use crate::parsers::traits::DomainParser;
use crate::parsers::codex_parser::{CodexParser, Rule};
use crate::ast::CodexResult;
use pest::Parser;

// Importamos los ASTs concretos para el casting
use crate::domains::optimization::OptimizationModel;
use crate::domains::boolean_algebra::BooleanModel;

pub struct CodexEngine {
    // HashMap<NombreDominio, Parser>
    registry: HashMap<String, Box<dyn DomainParser>>,
}

impl CodexEngine {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register<T: DomainParser + 'static>(&mut self, parser: T) {
        self.registry.insert(parser.domain_name().to_string(), Box::new(parser));
    }

    pub fn process_file(&self, content: &str) -> Vec<CodexResult> {
        let mut results = Vec::new();

        // 1. Parsing Global
        let pairs = match CodexParser::parse(Rule::program, content) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error crítico de sintaxis global: {}", e);
                return vec![];
            }
        };

        for pair in pairs {
            for inner in pair.into_inner() {
                if let Rule::domain_block = inner.as_rule() {
                    self.handle_domain_block(inner, &mut results);
                }
            }
        }
        results
    }

    // He extraído esta lógica a una función privada para que sea más legible
    fn handle_domain_block(&self, pair: pest::iterators::Pair<Rule>, results: &mut Vec<CodexResult>) {
        let mut parts = pair.into_inner();
        let domain_name = parts.next().unwrap().as_str();

        // Saltamos el string opcional (nombre del problema) si existe
        let mut next_part = parts.next().unwrap();
        if next_part.as_rule() == Rule::string_lit {
            next_part = parts.next().unwrap();
        }
        let raw_content = next_part.as_str();

        // 2. Dispatching
        if let Some(parser) = self.registry.get(domain_name) {
            match parser.parse_domain(raw_content) {
                Ok(any_ast) => self.convert_and_store(any_ast, results),
                Err(e) => eprintln!("Error en dominio '{}': {}", domain_name, e),
            }
        } else {
            eprintln!("Advertencia: No hay parser registrado para '{}'", domain_name);
        }
    }

    // 3. Conversión de Tipos (Any -> Enum)
    fn convert_and_store(&self, any_ast: Box<dyn Any>, results: &mut Vec<CodexResult>) {
        if let Some(model) = any_ast.downcast_ref::<OptimizationModel>() {
            // Nota: Necesitas #[derive(Clone)] en tus ASTs para esto
            results.push(CodexResult::Optimization(model.clone()));
        } 
        else if let Some(model) = any_ast.downcast_ref::<BooleanModel>() {
            results.push(CodexResult::Boolean(model.clone()));
        }
    }
}