use pest::Parser;
use pest_derive::Parser;
use crate::parsers::traits::{DomainParser, DomainResult};
use super::ast::{
    OptimizationBlock, OptimizationModel, 
    OptimizationDirection, ConstraintModel
};
use suma_core::symbolics::ast::{Expr, var};

#[derive(Parser)]
#[grammar = "domains/optimization/grammar.pest"]
pub struct OptimizationPestGrammar;

pub struct OptimizationParser;

impl DomainParser for OptimizationParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        // Solo respondemos a la definición del modelo
        vec!["Optimization"] 
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        let pairs = OptimizationPestGrammar::parse(Rule::optimization_block, content)
            .map_err(|e| format!("{}", e))?;

        if let Some(root) = pairs.clone().next() {
            // La gramática ahora solo debería permitir definiciones aquí
            // optimization_block = { definition }
            let inner = root.into_inner().next().unwrap(); 
            
            match inner.as_rule() {
                Rule::definition => {
                    let model = parse_definition(inner);
                    // Empaquetamos en el Enum de bloque
                    Ok(Box::new(OptimizationBlock::Definition(model)))
                },
                _ => Err(format!("Regla inesperada: {:?}", inner.as_rule()).into())
            }
        } else {
            Err("Bloque vacío".to_string().into())
        }
    }
}

// --- HELPERS ---

fn parse_definition(pair: pest::iterators::Pair<Rule>) -> OptimizationModel {
    let mut inner = pair.into_inner();

    // 1. ID
    let id_pair = inner.next().unwrap(); 
    let name = parse_string_lit(id_pair);

    // 2. Header
    let header_pair = inner.next().unwrap();
    let (direction, objective) = parse_header(header_pair);

    // 3. Constraints
    let mut constraints = Vec::new();
    if let Some(const_section) = inner.next() {
        for const_pair in const_section.into_inner() {
            if const_pair.as_rule() == Rule::constraint {
                constraints.push(parse_constraint(const_pair));
            }
        }
    }

    OptimizationModel { name, direction, objective, constraints }
}

fn parse_string_lit(pair: pest::iterators::Pair<Rule>) -> String {
    // Asumiendo estructura model_id -> string_lit
    if pair.as_rule() == Rule::string_lit {
        pair.as_str().trim_matches('"').to_string()
    } else {
        pair.into_inner().next().unwrap().as_str().trim_matches('"').to_string()
    }
}

fn parse_header(pair: pest::iterators::Pair<Rule>) -> (OptimizationDirection, Expr) {
    let mut inner = pair.into_inner();
    let dir_str = inner.next().unwrap().as_str();
    let direction = match dir_str.to_lowercase().as_str() {
        "maximize" | "max" => OptimizationDirection::Maximize,
        "minimize" | "min" => OptimizationDirection::Minimize,
        _ => panic!("Dirección desconocida"),
    };
    let objective = parse_expr(inner.next().unwrap());
    (direction, objective)
}

fn parse_constraint(pair: pest::iterators::Pair<Rule>) -> ConstraintModel {
    let mut inner = pair.into_inner();
    let left = parse_expr(inner.next().unwrap());
    let relation = inner.next().unwrap().as_str().to_string();
    let right = parse_expr(inner.next().unwrap());
    ConstraintModel { left, relation, right }
}

/// Parsea expresiones completas: A + B - C
fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    // Regla: expression = { term ~ (add_op ~ term)* }
    let mut inner = pair.into_inner();
    
    // 1. Parsear el primer término (Izquierda)
    let mut lhs = parse_term(inner.next().unwrap());

    // 2. Mientras haya operadores (+, -), seguir consumiendo términos
    while let Some(op) = inner.next() {
        let rhs = parse_term(inner.next().unwrap());
        lhs = match op.as_str() {
            "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
            "-" => Expr::Sub(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    lhs
}

/// Parsea términos multiplicativos: A * B / C
fn parse_term(pair: pest::iterators::Pair<Rule>) -> Expr {
    // Regla: term = { factor ~ (mul_op ~ factor)* }
    let mut inner = pair.into_inner();

    // 1. Parsear el primer factor
    let mut lhs = parse_factor(inner.next().unwrap());

    // 2. Mientras haya operadores (*, /)
    while let Some(op) = inner.next() {
        let rhs = parse_factor(inner.next().unwrap());
        lhs = match op.as_str() {
            "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
            "/" => Expr::Div(Box::new(lhs), Box::new(rhs)),
            _ => unreachable!(),
        };
    }
    lhs
}

/// Parsea factores y unarios: -A, 5, x, (A+B)
fn parse_factor(pair: pest::iterators::Pair<Rule>) -> Expr {
    // Regla: factor = { neg_op? ~ atom }
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    // Verificamos si hay un signo negativo primero
    if first.as_rule() == Rule::neg_op {
        let atom_pair = inner.next().unwrap();
        let atom_expr = parse_atom(atom_pair);
        return Expr::Neg(Box::new(atom_expr));
    } else {
        // Si no hay signo, el 'first' ya es el atom (o lo que pest haya envuelto)
        // Nota: Como 'atom' es silencioso (_) en la gramática, Pest puede devolver directamenete
        // number, variable o expression.
        return parse_atom(first);
    }
}

/// Parsea el valor real: número, variable o paréntesis
fn parse_atom(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::number => Expr::Const(pair.as_str().parse().unwrap_or(0.0)),
        Rule::variable => var(pair.as_str()),
        Rule::expression => parse_expr(pair), // Recursión para paréntesis (A+B)
        _ => {
            // Manejo de caso borde donde Pest envuelve el atom
            // Si llega aquí con regla 'atom' (aunque es silenciosa), profundizamos.
            let inner = pair.into_inner().next().unwrap();
            parse_atom(inner)
        }
    }
}