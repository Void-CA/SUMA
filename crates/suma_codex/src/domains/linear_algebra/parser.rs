use pest::Parser;
use pest_derive::Parser;

use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::{LinearAlgebraModel, LinearAlgebraAction, SystemDef, AnalysisDef, MatrixData};

#[derive(Parser)]
#[grammar = "domains/linear_algebra/grammar.pest"]
pub struct LinearAlgebraPestGrammar;

pub struct LinearAlgebraParser;

impl DomainParser for LinearAlgebraParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec![
            "LinearSystem", // Definiciones
            "Analysis"      // Queries
        ]
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        // 1. Parsear el contenido del bloque
        let pairs = LinearAlgebraPestGrammar::parse(Rule::linear_algebra_block, content)
            .map_err(|e| format!("{}", e))?;

        let mut actions = Vec::new();

        // 2. Iterar sobre las definiciones (System o Analysis)
        if let Some(root) = pairs.clone().next() {
            for pair in root.into_inner() {
                match pair.as_rule() {
                    Rule::system_def => {
                        actions.push(LinearAlgebraAction::System(parse_system(pair)));
                    },
                    Rule::analysis_def => {
                        actions.push(LinearAlgebraAction::Analysis(parse_analysis(pair)));
                    },
                    _ => {}
                }
            }
        }

        // Retornamos el modelo contenedor
        Ok(Box::new(LinearAlgebraModel { 
            name: None, // Se llenará en el Dispatcher
            actions 
        }))
    }
}

// --- Helpers de Parsing ---

fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut inner = pair.into_inner();
    let name = parse_name(inner.next().unwrap());
    
    let mut sys = SystemDef { name, matrix_a: None, vector_b: None };

    for prop in inner {
        // prop es 'sys_property', su hijo es 'prop_a' o 'prop_b'
        let specific_prop = prop.into_inner().next().unwrap();
        
        match specific_prop.as_rule() {
            Rule::prop_a => {
                // Estructura: "coefficients" ~ ":" ~ matrix_lit
                // El literal se ignora, tomamos el matrix_lit
                let matrix_pair = specific_prop.into_inner().next().unwrap();
                sys.matrix_a = Some(parse_matrix(matrix_pair));
            },
            Rule::prop_b => {
                let matrix_pair = specific_prop.into_inner().next().unwrap();
                sys.vector_b = Some(parse_matrix(matrix_pair));
            },
            _ => {}
        }
    }
    sys
}

fn parse_analysis(pair: pest::iterators::Pair<Rule>) -> AnalysisDef {
    let mut inner = pair.into_inner();
    let name = parse_name(inner.next().unwrap());

    let mut analysis = AnalysisDef { 
        name, 
        target: String::new(), 
        calculate: Vec::new(), 
        export: Vec::new() 
    };

    for prop in inner {
        // prop es 'query_property', su hijo es prop_target, prop_calculate, etc.
        let specific_prop = prop.into_inner().next().unwrap();

        match specific_prop.as_rule() {
            Rule::prop_target => {
                // Estructura: "target" ~ ":" ~ name_ref
                let val_pair = specific_prop.into_inner().next().unwrap();
                analysis.target = parse_name(val_pair);
            },
            Rule::prop_calculate => {
                // Estructura: "calculate" ~ ":" ~ prop_list
                let list_pair = specific_prop.into_inner().next().unwrap();
                for item in list_pair.into_inner() {
                    analysis.calculate.push(item.as_str().to_string());
                }
            },
            Rule::prop_export => {
                let map_pair = specific_prop.into_inner().next().unwrap();
                for item in map_pair.into_inner() {
                    let mut parts = item.into_inner();
                    let prop = parts.next().unwrap().as_str().to_string();
                    let var = parts.next().unwrap().as_str().to_string();
                    analysis.export.push((prop, var));
                }
            },
            _ => {}
        }
    }
    analysis
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

fn parse_name(pair: pest::iterators::Pair<Rule>) -> String {
    pair.as_str().trim_matches('"').to_string()
}