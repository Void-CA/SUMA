use std::collections::HashMap;

use crate::parsers::traits::DomainParser;
use crate::parsers::codex_parser::{CodexParser, Rule};
use crate::ast::CodexResult;
use pest::Parser;

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
        let full_text = pair.as_str(); 
        let mut parts = pair.clone().into_inner();
        let keyword = parts.next().unwrap().as_str();

        if let Some(&index) = self.routes.get(keyword) {
            let parser = &self.parsers[index];
            
            match parser.parse_domain(full_text) { 
                Ok(result) => results.push(result),
                Err(e) => println!("Error in block '{}': {}", keyword, e),
            }
        } else {
            eprintln!("Warning: Unknown keyword '{}'. Did you forget to register the domain?", keyword);
        }
    }
}