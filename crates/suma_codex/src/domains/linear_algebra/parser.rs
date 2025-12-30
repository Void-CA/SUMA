use pest::Parser;
use pest_derive::Parser;

use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::{
    LinearAlgebraBlock, LinAlgStmt, SystemDef, MatrixData
};

#[derive(Parser)]
#[grammar = "domains/linear_algebra/grammar.pest"]
pub struct LinearAlgebraPestGrammar;

pub struct LinearAlgebraParser;

impl DomainParser for LinearAlgebraParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        // Solo respondemos a definiciones de sistemas
        vec!["LinearSystem"] 
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        let pairs = LinearAlgebraPestGrammar::parse(Rule::linear_algebra_block, content)
            .map_err(|e| format!("{}", e))?;

        let mut statements = Vec::new();

        if let Some(root) = pairs.clone().next() {
            for pair in root.into_inner() {
                match pair.as_rule() {
                    Rule::system_def => {
                        // Solo parseamos definiciones
                        statements.push(LinAlgStmt::System(parse_system(pair)));
                    },
                    // Regla Query eliminada. El parser genérico se encarga ahora.
                    _ => {}
                }
            }
        }

        Ok(Box::new(LinearAlgebraBlock { 
            statements 
        }))
    }
}

// --- HELPERS (Solo Definición) ---

fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut inner = pair.into_inner();
    
    let id = parse_string_lit(inner.next().unwrap());
    
    let mut sys = SystemDef { 
        id, 
        coefficients: None, 
        constants: None 
    };

    for prop in inner {
        let specific_prop = prop.into_inner().next().unwrap();
        
        match specific_prop.as_rule() {
            Rule::prop_coeffs => {
                let matrix_pair = specific_prop.into_inner().next().unwrap();
                sys.coefficients = Some(parse_matrix(matrix_pair));
            },
            Rule::prop_consts => {
                let matrix_pair = specific_prop.into_inner().next().unwrap();
                sys.constants = Some(parse_matrix(matrix_pair));
            },
            _ => {}
        }
    }
    sys
}

fn parse_matrix(pair: pest::iterators::Pair<Rule>) -> MatrixData {
    let mut data = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    for row_pair in pair.into_inner() {
        rows += 1;
        let mut current_cols = 0;
        for num_pair in row_pair.into_inner() {
            let val: f64 = num_pair.as_str().parse().unwrap_or(0.0);
            data.push(val);
            current_cols += 1;
        }
        cols = current_cols;
    }
    MatrixData { rows, cols, data }
}

fn parse_string_lit(pair: pest::iterators::Pair<Rule>) -> String {
    pair.as_str().trim_matches('"').to_string()
}