use pest::Parser;
use pest_derive::Parser;
use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::{QueryBlock, QueryCommand};

#[derive(Parser)]
#[grammar = "domains/queries/grammar.pest"]
pub struct QueryPestGrammar;

pub struct QueryParser;

impl DomainParser for QueryParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec!["query"] // ESTE será el único dueño de la palabra "query"
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        let pairs = QueryPestGrammar::parse(Rule::query_block, content)
            .map_err(|e| format!("{}", e))?;

        if let Some(root) = pairs.clone().next() {
            let mut inner = root.into_inner();
            
            // 1. Target
            let target_str = inner.next().unwrap().as_str(); // "ID"
            let target_id = target_str.trim_matches('"').to_string();

            // 2. Comandos
            let mut commands = Vec::new();
            for pair in inner { // iteramos sobre reglas 'command'
                let mut cmd_inner = pair.into_inner();
                
                // Acción
                let action = cmd_inner.next().unwrap().as_str().to_string();
                
                // Alias (Opcional)
                let alias = if let Some(alias_pair) = cmd_inner.next() {
                    // alias_pair es "as X", queremos el X (segundo token dentro)
                    // Ojo con la gramática: alias_clause = { "as" ~ identifier }
                    // inner de alias_clause tiene el identifier.
                    Some(alias_pair.into_inner().next().unwrap().as_str().to_string())
                } else {
                    None
                };

                commands.push(QueryCommand { action, alias });
            }

            Ok(Box::new(QueryBlock { target_id, commands }))
        } else {
            Err("Query inválida".to_string().into())
        }
    }
}