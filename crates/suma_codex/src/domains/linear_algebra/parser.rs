use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::{LinearAlgebraModel, LinAlgStmt, LinAlgExpr};

#[derive(Parser)]
#[grammar = "domains/linear_algebra/grammar.pest"]
pub struct LinearAlgebraPestGrammar;

pub struct LinearAlgebraParser;

impl DomainParser for LinearAlgebraParser {
    fn domain_name(&self) -> &'static str { "linear_algebra" }

    fn parse_domain(&self, content: &str) -> DomainResult {
        // 1. Parsing crudo con Pest
        let mut pairs = LinearAlgebraPestGrammar::parse(Rule::linear_algebra_block, content)
            .map_err(|e| format!("Error de sintaxis en Álgebra Lineal: {}", e))?;

        let mut statements = Vec::new();

        // 2. Recorrer el árbol de Pest
        if let Some(root) = pairs.next() {
            for pair in root.into_inner() {
                match pair.as_rule() {
                    Rule::stmt => {
                        let inner = pair.into_inner().next().unwrap();
                        match inner.as_rule() {
                            Rule::assignment => statements.push(parse_assignment(inner)),
                            Rule::expr_stmt => {
                                let expr_pair = inner.into_inner().next().unwrap();
                                statements.push(LinAlgStmt::Evaluation(parse_expr(expr_pair)));
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        // Retornamos el modelo inicializado (name: None se llenará en el Dispatcher)
        Ok(Box::new(LinearAlgebraModel {
            name: None,
            statements
        }))
    }
}

// --- Funciones Auxiliares de Parsing ---

fn parse_assignment(pair: Pair<Rule>) -> LinAlgStmt {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let expr_pair = inner.next().unwrap();
    LinAlgStmt::Assignment {
        target: ident,
        value: parse_expr(expr_pair),
    }
}

fn parse_expr(pair: Pair<Rule>) -> LinAlgExpr {
    // Manejo básico de precedencia: expr -> term -> (add_op term)*
    let mut inner = pair.into_inner();
    let mut lhs = parse_term(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let op = op_pair.as_str().to_string();
        let rhs = parse_term(inner.next().unwrap());
        lhs = LinAlgExpr::BinaryOp {
            lhs: Box::new(lhs),  // Antes left
            op,
            rhs: Box::new(rhs),  // Antes right
        };
    }
    lhs
}

fn parse_term(pair: Pair<Rule>) -> LinAlgExpr {
    // term -> factor -> (mul_op factor)*
    let mut inner = pair.into_inner();
    let mut lhs = parse_factor(inner.next().unwrap());

    while let Some(op_pair) = inner.next() {
        let op = op_pair.as_str().to_string();
        let rhs = parse_factor(inner.next().unwrap());
        lhs = LinAlgExpr::BinaryOp {
            lhs: Box::new(lhs),  // Antes left
            op,
            rhs: Box::new(rhs),  // Antes right
        };
    }
    lhs
}

fn parse_factor(pair: Pair<Rule>) -> LinAlgExpr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => LinAlgExpr::Number(inner.as_str().parse().unwrap_or(0.0)),
        Rule::ident => LinAlgExpr::Variable(inner.as_str().to_string()),
        Rule::matrix_lit => parse_matrix(inner),
        Rule::call => parse_call(inner),
        Rule::expr => parse_expr(inner), // Paréntesis: recursión indirecta
        _ => unreachable!("Factor desconocido en gramática: {:?}", inner.as_rule()),
    }
}

fn parse_matrix(pair: Pair<Rule>) -> LinAlgExpr {
    let mut rows = Vec::new();
    for row_pair in pair.into_inner() {
        let mut row_exprs = Vec::new();
        for expr_pair in row_pair.into_inner() {
            row_exprs.push(parse_expr(expr_pair));
        }
        rows.push(row_exprs);
    }
    LinAlgExpr::Matrix(rows)
}

fn parse_call(pair: Pair<Rule>) -> LinAlgExpr {
    let mut inner = pair.into_inner();
    let func_name = inner.next().unwrap().as_str().to_string();
    let mut args = Vec::new();

    if let Some(args_pair) = inner.next() {
        for arg in args_pair.into_inner() {
            args.push(parse_expr(arg));
        }
    }

    LinAlgExpr::Call { function: func_name, args }
}