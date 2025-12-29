use std::collections::HashMap;
use std::any::Any;

use crate::parsers::traits::DomainParser;
use crate::parsers::codex_parser::{CodexParser, Rule};
use crate::ast::CodexResult;
use pest::Parser;

// Importamos los Modelos de los dominios
use crate::domains::optimization::OptimizationModel;
use crate::domains::boolean_algebra::BooleanModel;
// CAMBIO CRÍTICO: Importamos el nuevo Block en lugar del Model antiguo
use crate::domains::linear_algebra::ast::LinearAlgebraBlock;

pub struct CodexEngine {
    parsers: Vec<Box<dyn DomainParser>>,
    routes: HashMap<String, usize>,
}

impl CodexEngine {
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
            routes: HashMap::new(),
        }
    }

    pub fn register<T: DomainParser + 'static>(&mut self, parser: T) {
        let index = self.parsers.len();
        let keywords = parser.valid_keywords();
        for kw in keywords {
            self.routes.insert(kw.to_string(), index);
        }
        self.parsers.push(Box::new(parser));
    }

    pub fn process_file(&self, content: &str) -> Vec<CodexResult> {
        let mut results = Vec::new();

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

    fn handle_domain_block(&self, pair: pest::iterators::Pair<Rule>, results: &mut Vec<CodexResult>) {
        // 1. CAPTURAR EL TEXTO COMPLETO
        let full_text = pair.as_str(); 
        
        // 2. IDENTIFICAR LA KEYWORD PARA EL RUTEO
        // Si viene 'query "X" {...}', la keyword es "query".
        // Como registramos "query" en LinearAlgebraParser, el ruteo funciona solo.
        let mut parts = pair.clone().into_inner();
        let keyword = parts.next().unwrap().as_str();

        // 3. RUTEO
        if let Some(&index) = self.routes.get(keyword) {
            let parser = &self.parsers[index];
            
            match parser.parse_domain(full_text) { 
                Ok(any_ast) => {
                    self.convert_and_store(any_ast, results)
                }
                Err(e) => println!("Error en bloque '{}': {}", keyword, e),
            }
        } else {
            eprintln!("Advertencia: Palabra clave desconocida '{}'. ¿Olvidaste registrar el dominio?", keyword);
        }
    }

    fn convert_and_store(&self, any_ast: Box<dyn Any>, results: &mut Vec<CodexResult>) {
        if let Some(model) = any_ast.downcast_ref::<OptimizationModel>() {
            results.push(CodexResult::Optimization(model.clone()));
        } 
        else if let Some(model) = any_ast.downcast_ref::<BooleanModel>() {
            results.push(CodexResult::Boolean(model.clone()));
        }
        // CAMBIO CRÍTICO: Downcast al nuevo LinearAlgebraBlock
        else if let Some(block) = any_ast.downcast_ref::<LinearAlgebraBlock>() {
            results.push(CodexResult::LinearAlgebra(block.clone()));
        }
    }
}