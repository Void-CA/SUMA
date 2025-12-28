use pest::Parser;
use pest_derive::Parser;
use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::*; // <--- IMPORTANTE: Importamos todo del archivo ast.rs

#[derive(Parser)]
#[grammar = "domains/optimization/grammar.pest"]
pub struct OptimizationPestGrammar;

pub struct OptimizationParser;

impl DomainParser for OptimizationParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec![
            "Optimization"
        ]
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        let mut pairs = OptimizationPestGrammar::parse(Rule::optimize_block, content)
            .map_err(|e| format!("Error parsing optimization: {}", e))?;

        let mut model = OptimizationModel {
            name: None,
            obj_type: OptType::Maximize,
            obj_expr: Expr::Number(0.0),
            constraints: Vec::new(),
        };

        let block_pair = pairs.next().unwrap(); // optimize_block

        for inner in block_pair.into_inner() {
            match inner.as_rule() {
                Rule::objective => process_objective(inner, &mut model),
                Rule::constraints_section => process_constraints_section(inner, &mut model),
                _ => {}
            }
        }

        Ok(Box::new(model))
    }
}

// --- Helpers ---

// Parsea: term + term - term ...
fn build_expression(pair: pest::iterators::Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    
    // 1. Obtener el primer término (la izquierda)
    let mut lhs = build_term(inner.next().unwrap());

    // 2. Mientras haya más pares (operador, término), seguimos anidando
    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            _ => unreachable!(),
        };
        
        let rhs_pair = inner.next().unwrap();
        let rhs = build_term(rhs_pair);

        // Envolvemos lo que llevamos en una nueva operación
        lhs = Expr::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    lhs
}

// Parsea: factor * factor / factor ...
fn build_term(pair: pest::iterators::Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let mut lhs = build_factor(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => unreachable!(),
        };

        let rhs = build_factor(inner.next().unwrap());
        
        lhs = Expr::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    lhs
}

// Parsea: número, variable, o ( expresión )
fn build_factor(pair: pest::iterators::Pair<Rule>) -> Expr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => {
            let val = inner.as_str().parse::<f64>().unwrap_or(0.0);
            Expr::Number(val)
        },
        Rule::ident => {
            Expr::Variable(inner.as_str().to_string())
        },
        Rule::expression => {
            // Paréntesis: volvemos a llamar a build_expression recursivamente
            build_expression(inner)
        },
        _ => unreachable!("Factor desconocido: {:?}", inner.as_rule()),
    }
}

// Helper para la sección de objetivo
fn process_objective(pair: pest::iterators::Pair<Rule>, model: &mut OptimizationModel) {
    // Ya sabemos que 'pair' es Rule::objective, iteramos sus hijos
    for field in pair.into_inner() {
        match field.as_rule() {
            Rule::type_obj => {
                model.obj_type = match field.as_str() {
                    "minimize" => OptType::Minimize,
                    _ => OptType::Maximize,
                };
            },
            Rule::expression => {
                model.obj_expr = build_expression(field);
            },
            _ => {}
        }
    }
}

// Helper para la sección de restricciones
fn process_constraints_section(pair: pest::iterators::Pair<Rule>, model: &mut OptimizationModel) {
    // Estructura: constraints_section -> constraints -> constraint
    // Navegamos hacia abajo
    for list_node in pair.into_inner() { // Entra a 'constraints' (la lista)
        for constraint_node in list_node.into_inner() { // Entra a cada 'constraint'
             // Otra delegación para mantener esto limpio
             let constraint = build_constraint(constraint_node);
             model.constraints.push(constraint);
        }
    }
}

// Helper para construir UNA sola restricción
fn build_constraint(pair: pest::iterators::Pair<Rule>) -> Constraint {
    let mut parts = pair.into_inner();
    
    // Al aislar esto, la lógica se ve mucho más clara:
    let lhs = build_expression(parts.next().unwrap());
    let op = parts.next().unwrap().as_str().to_string();
    let rhs = build_expression(parts.next().unwrap());

    Constraint { lhs, op, rhs }
}

