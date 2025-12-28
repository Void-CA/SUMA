use pest::Parser;
use pest_derive::Parser;
use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::*;

#[derive(Parser)]
#[grammar = "domains/boolean_algebra/grammar.pest"]
pub struct BooleanPestGrammar;

pub struct BooleanParser;

impl DomainParser for BooleanParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec!["Boolean"]
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        // Parseamos SOLO la expresión interna del bloque
        let pairs = BooleanPestGrammar::parse(Rule::boolean_block, content)
            .map_err(|e| format!("Error en lógica booleana: {}", e))?;

        let root_pair = pairs.into_iter().next().unwrap(); // boolean_block

        let mut model = BooleanModel {
            root: BoolExpr::Literal(false), // temporal
            name: None,                     // nombre lo asigna el dispatcher
        };

        // Iteramos sobre los hijos de boolean_block
        for inner_pair in root_pair.into_inner() {
            if inner_pair.as_rule() == Rule::expression {
                model.root = build_expression(inner_pair);
            }
        }

        Ok(Box::new(model))
    }
}

// --- Builders Recursivos (Precedencia OR -> AND -> NOT) ---

fn build_expression(pair: pest::iterators::Pair<Rule>) -> BoolExpr {
    let mut inner = pair.into_inner();
    let mut lhs = build_term(inner.next().unwrap());

    while let Some(_) = inner.next() { // El operador (lo ignoramos pq sabemos que es OR)
        let rhs = build_term(inner.next().unwrap());
        lhs = BoolExpr::BinaryOp {
            op: BoolOp::Or,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    lhs
}

fn build_term(pair: pest::iterators::Pair<Rule>) -> BoolExpr {
    let mut inner = pair.into_inner();
    let mut lhs = build_factor(inner.next().unwrap());

    while let Some(_) = inner.next() { // El operador (AND)
        let rhs = build_factor(inner.next().unwrap());
        lhs = BoolExpr::BinaryOp {
            op: BoolOp::And,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    lhs
}

fn build_factor(pair: pest::iterators::Pair<Rule>) -> BoolExpr {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    // Verificamos si hay un NOT
    if first.as_rule() == Rule::not_op {
        let atom = build_atom(inner.next().unwrap());
        return BoolExpr::Not(Box::new(atom));
    } else {
        // Si no hay NOT, el "first" ya es el atom
        return build_atom(first);
    }
}

fn build_atom(pair: pest::iterators::Pair<Rule>) -> BoolExpr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::boolean => {
            let val = inner.as_str().eq_ignore_ascii_case("true");
            BoolExpr::Literal(val)
        },
        Rule::ident => BoolExpr::Variable(inner.as_str().to_string()),
        Rule::expression => build_expression(inner), // Paréntesis
        _ => unreachable!(),
    }
}