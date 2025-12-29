use pest::Parser;
use pest_derive::Parser;

use crate::parsers::traits::{DomainParser, DomainResult};
// Importamos el nuevo AST refactorizado
use super::ast::{
    LinearAlgebraBlock, LinAlgStmt, SystemDef, QueryDef, 
    Request, Capability, MatrixData
};

#[derive(Parser)]
#[grammar = "domains/linear_algebra/grammar.pest"]
pub struct LinearAlgebraPestGrammar;

pub struct LinearAlgebraParser;

impl DomainParser for LinearAlgebraParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec![
            "LinearSystem", // Definidor de estado
            "query"         // Ejecutor
        ]
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        // 1. Iniciar parsing con la regla raíz
        let pairs = LinearAlgebraPestGrammar::parse(Rule::linear_algebra_block, content)
            .map_err(|e| format!("{}", e))?;

        let mut statements = Vec::new();

        // 2. Iterar sobre sentencias (System o Query)
        if let Some(root) = pairs.clone().next() {
            for pair in root.into_inner() {
                match pair.as_rule() {
                    Rule::system_def => {
                        statements.push(LinAlgStmt::System(parse_system(pair)));
                    },
                    Rule::query_def => {
                        statements.push(LinAlgStmt::Query(parse_query(pair)));
                    },
                    _ => {}
                }
            }
        }

        // Retornamos el bloque contenedor plano
        Ok(Box::new(LinearAlgebraBlock { 
            statements 
        }))
    }
}

// ==========================================
// HELPERS DE PARSING (Detalle de Implementación)
// ==========================================

/// Parsea: LinearSystem "ID" { coefficients: [...], constants: [...] }
fn parse_system(pair: pest::iterators::Pair<Rule>) -> SystemDef {
    let mut inner = pair.into_inner();
    
    // 1. Extraer ID (model_id es un string_lit)
    let id = parse_string_lit(inner.next().unwrap());
    
    let mut sys = SystemDef { 
        id, 
        coefficients: None, 
        constants: None 
    };

    // 2. Iterar propiedades
    for prop in inner {
        // prop es 'sys_property', hijos: 'prop_coeffs' o 'prop_consts'
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

/// Parsea: query "ID" { capability [as alias], ... }
fn parse_query(pair: pest::iterators::Pair<Rule>) -> QueryDef {
    let mut inner = pair.into_inner();
    
    // 1. Extraer Target (target_ref es un string_lit)
    let target_id = parse_string_lit(inner.next().unwrap());

    let mut requests = Vec::new();

    // 2. Iterar sentencias de la query
    for stmt in inner {
        if stmt.as_rule() == Rule::query_stmt {
            requests.push(parse_request(stmt));
        }
    }

    QueryDef { target_id, requests }
}

/// Parsea una linea: "determinant as det_A"
fn parse_request(pair: pest::iterators::Pair<Rule>) -> Request {
    let mut inner = pair.into_inner();
    
    // 1. Capability (Obligatorio)
    let cap_pair = inner.next().unwrap();
    let capability = match cap_pair.as_str() {
        "determinant" => Capability::Determinant,
        "solution"    => Capability::Solution,
        "inverse"     => Capability::Inverse,
        "rank"        => Capability::Rank,
        "trace"       => Capability::Trace,
        _ => panic!("Capability desconocida en parser: {}", cap_pair.as_str()),
    };

    // 2. Alias (Opcional)
    // La gramática dice: capability ~ (k_as ~ alias_ident)?
    // Si hay un siguiente token, es el alias_ident (el 'as' es silencioso/oculto en pest si tiene _)
    let alias = if let Some(alias_pair) = inner.next() {
        Some(alias_pair.as_str().to_string())
    } else {
        None
    };

    Request { capability, alias }
}

/// Parsea Matrices (Lógica matemática pura)
fn parse_matrix(pair: pest::iterators::Pair<Rule>) -> MatrixData {
    let mut data = Vec::new();
    let mut rows = 0;
    let mut cols = 0;

    for row_pair in pair.into_inner() {
        rows += 1;
        let mut current_cols = 0;
        for num_pair in row_pair.into_inner() {
            // Manejo robusto de números flotantes
            let val: f64 = num_pair.as_str().parse().unwrap_or(0.0);
            data.push(val);
            current_cols += 1;
        }
        cols = current_cols;
    }
    MatrixData { rows, cols, data }
}

/// Utilidad para limpiar comillas de string_lit: "Texto" -> Texto
fn parse_string_lit(pair: pest::iterators::Pair<Rule>) -> String {
    pair.as_str().trim_matches('"').to_string()
}