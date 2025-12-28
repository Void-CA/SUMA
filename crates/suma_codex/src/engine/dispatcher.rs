use std::collections::HashMap;
use std::any::Any;

// Imports internos
use crate::parsers::traits::DomainParser;
use crate::parsers::codex_parser::{CodexParser, Rule};
use crate::ast::CodexResult;
use pest::Parser;

// Importamos los ASTs concretos
use crate::domains::optimization::OptimizationModel;
use crate::domains::boolean_algebra::BooleanModel;
use crate::domains::linear_algebra::ast::LinearAlgebraModel;

pub struct CodexEngine {
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
                // Buscamos 'domain_block' según la regla actualizada
                if let Rule::domain_block = inner.as_rule() {
                    self.handle_domain_block(inner, &mut results);
                }
            }
        }
        results
    }

    fn handle_domain_block(&self, pair: pest::iterators::Pair<Rule>, results: &mut Vec<CodexResult>) {
        let mut parts = pair.into_inner();
        
        // 1. Identificador del dominio (ej: "linear_algebra")
        // La gramática es: ident ~ name? ~ "{" ~ domain_content ~ "}"
        let domain_name = parts.next().unwrap().as_str();

        // 2. Lógica para detectar si hay Nombre o directo Contenido
        let next_pair = parts.next().unwrap();
        
        let (name, raw_content) = match next_pair.as_rule() {
            Rule::name => {
                // CASO A: Hay nombre (ej: "Math_Real")
                // Quitamos comillas si es string_lit, o raw si es ident
                let n = next_pair.as_str().trim_matches('"').to_string();
                
                // El siguiente nodo OBLIGATORIAMENTE es el contenido
                let content_pair = parts.next().unwrap();
                (Some(n), content_pair.as_str())
            },
            Rule::domain_content => {
                // CASO B: No hay nombre, pasamos directo al contenido
                (None, next_pair.as_str())
            },
            _ => {
                // Caso defensivo
                (None, "")
            }
        };

        // 3. Dispatch al Parser Específico
        if let Some(parser) = self.registry.get(domain_name) {
            match parser.parse_domain(raw_content) {
                Ok(mut any_ast) => {
                    // Inyectar el nombre global al modelo si existe
                    if let Some(n) = name {
                        if let Some(opt_model) = any_ast.downcast_mut::<OptimizationModel>() {
                            opt_model.name = Some(n);
                        } else if let Some(bool_model) = any_ast.downcast_mut::<BooleanModel>() {
                            bool_model.name = Some(n);
                        } else if let Some(lin_model) = any_ast.downcast_mut::<LinearAlgebraModel>() {
                            lin_model.name = Some(n);
                        }
                    }
                    self.convert_and_store(any_ast, results)
                }
                Err(e) => {
                    // Importante: println para ver el error en tests --nocapture
                    println!("Error en dominio '{}': {}", domain_name, e);
                },
            }
        } else {
            eprintln!("Advertencia: No hay parser registrado para '{}'", domain_name);
        }
    }

    // 4. Conversión y Almacenamiento
    fn convert_and_store(&self, any_ast: Box<dyn Any>, results: &mut Vec<CodexResult>) {
        if let Some(model) = any_ast.downcast_ref::<OptimizationModel>() {
            results.push(CodexResult::Optimization(model.clone()));
        } 
        else if let Some(model) = any_ast.downcast_ref::<BooleanModel>() {
            results.push(CodexResult::Boolean(model.clone()));
        }
        else if let Some(model) = any_ast.downcast_ref::<LinearAlgebraModel>() {
            results.push(CodexResult::LinearAlgebra(model.clone()));
        }
    }
}