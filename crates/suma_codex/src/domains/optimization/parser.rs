use pest::Parser;
use pest_derive::Parser;

use crate::parsers::traits::{DomainParser, DomainResult};
// Importamos el AST del dominio y la Expresión simbólica compartida
use super::ast::{
    OptimizationBlock, OptimizationModel, OptimizationDirection, ConstraintModel
};
use suma_core::symbolics::ast::{Expr, var}; // Asumiendo que 'var' es tu constructor helper

#[derive(Parser)]
#[grammar = "domains/optimization/grammar.pest"]
pub struct OptimizationPestGrammar;

pub struct OptimizationParser;

impl DomainParser for OptimizationParser {
    fn valid_keywords(&self) -> Vec<&'static str> {
        vec![
            "Optimization" // La palabra clave que activa este parser
        ]
    }

    fn parse_domain(&self, content: &str) -> DomainResult {
        // 1. Parsing inicial con Pest
        let pairs = OptimizationPestGrammar::parse(Rule::optimization_block, content)
            .map_err(|e| format!("{}", e))?;

        // 2. Extraer el bloque raíz
        // La gramática es: "optimization" ~ "{" ~ header ~ constraints? ~ "}"
        if let Some(root) = pairs.clone().next() {
            let model = parse_optimization_model(root);
            
            // Retornamos el bloque contenedor (Wrapper para el Engine)
            Ok(Box::new(OptimizationBlock { 
                model 
            }))
        } else {
            Err("No se encontró un bloque de optimización válido".to_string().into())
        }
    }
}

// ==========================================
// HELPERS DE PARSING (Mapeo Pest -> AST)
// ==========================================

fn parse_optimization_model(pair: pest::iterators::Pair<Rule>) -> OptimizationModel {
    let mut inner = pair.into_inner();

    // 1. Parsear Nombre Opcional
    // Miramos el siguiente token sin consumirlo (peek) o consumimos si coincide
    let first_rule = inner.peek().unwrap().as_rule();
    
    let name = if first_rule == Rule::model_id {
        // Consumimos el token 'model_id'
        let id_pair = inner.next().unwrap();
        // into_inner para entrar a 'string_lit', as_str, trim comillas
        Some(id_pair.into_inner().next().unwrap().as_str().trim_matches('"').to_string())
    } else {
        None
    };

    // 2. Header (Ahora será el siguiente token disponible)
    let header_pair = inner.next().unwrap();
    let (direction, objective) = parse_header(header_pair);

    // 3. Restricciones
    let mut constraints = Vec::new();
    if let Some(const_section) = inner.next() {
        for const_pair in const_section.into_inner() {
            if const_pair.as_rule() == Rule::constraint {
                constraints.push(parse_constraint(const_pair));
            }
        }
    }

    OptimizationModel {
        id: name.unwrap_or_else(|| "Unnamed_Model".to_string()), // Guardamos el nombre
        direction,
        objective,
        constraints,
    }
}

fn parse_header(pair: pest::iterators::Pair<Rule>) -> (OptimizationDirection, Expr) {
    let mut inner = pair.into_inner();
    
    // a. Dirección
    let dir_str = inner.next().unwrap().as_str();
    let direction = match dir_str {
        "maximize" | "max" => OptimizationDirection::Maximize,
        "minimize" | "min" => OptimizationDirection::Minimize,
        _ => panic!("Dirección desconocida en parser: {}", dir_str),
    };

    // b. Función Objetivo
    let expr_pair = inner.next().unwrap();
    let objective = parse_expr(expr_pair);

    (direction, objective)
}

fn parse_constraint(pair: pest::iterators::Pair<Rule>) -> ConstraintModel {
    let mut inner = pair.into_inner();

    // Estructura: expression ~ relation ~ expression
    let left = parse_expr(inner.next().unwrap());
    let relation = inner.next().unwrap().as_str().to_string();
    let right = parse_expr(inner.next().unwrap());

    ConstraintModel {
        left,
        relation,
        right,
    }
}

// ==========================================
// PARSING RECURSIVO DE EXPRESIONES
// ==========================================
// Transforma la gramática jerárquica (Expr -> Term -> Factor) en el árbol AST

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    // La regla 'expression' maneja sumas y restas: term ~ (add_op ~ term)*
    match pair.as_rule() {
        Rule::expression => {
            let mut inner = pair.into_inner();
            let mut lhs = parse_expr(inner.next().unwrap()); // Primer término

            // Mientras haya más pares (operador, término)
            while let Some(op) = inner.next() {
                let rhs = parse_expr(inner.next().unwrap());
                lhs = match op.as_str() {
                    "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
                    "-" => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                };
            }
            lhs
        },
        // La regla 'term' maneja multiplicación y división: factor ~ (mul_op ~ factor)*
        Rule::term => {
            let mut inner = pair.into_inner();
            let mut lhs = parse_expr(inner.next().unwrap());

            while let Some(op) = inner.next() {
                let rhs = parse_expr(inner.next().unwrap());
                lhs = match op.as_str() {
                    "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                    "/" => Expr::Div(Box::new(lhs), Box::new(rhs)),
                    _ => unreachable!(),
                };
            }
            lhs
        },
        // La regla 'factor' maneja unarios, átomos y paréntesis
        Rule::factor => {
            let mut inner = pair.into_inner();
            let first = inner.next().unwrap();

            if first.as_rule() == Rule::neg_op {
                // Caso unario: "-" ~ atom
                let atom = inner.next().unwrap();
                Expr::Neg(Box::new(parse_expr(atom))) // Recursión indirecta
            } else {
                // Caso directo: atom
                parse_expr(first)
            }
        },
        // Caso base: Números
        Rule::number => {
            let val: f64 = pair.as_str().parse().unwrap_or(0.0);
            Expr::Const(val)
        },
        // Caso base: Variables
        Rule::variable => {
            var(pair.as_str())
        },
        // Caso especial: Pest a veces devuelve la regla 'atom' que contiene 'expression' (paréntesis)
        // atom = _{ number | variable | "(" ~ expression ~ ")" }
        // Como 'atom' es silencioso (_), recibiremos directamente number, variable o expression.
        // Si por alguna razón recibimos una expresión anidada aquí, recursamos.
        _ => parse_expr(pair), 
    }
}