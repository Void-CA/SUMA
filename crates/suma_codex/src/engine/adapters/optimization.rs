use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::Write;

// Core Imports
use suma_core::optimization::linear::model::{
    LinearProblem, Objective, Constraint, LinearExpression, Relation, OptimizationDirection as CoreDirection
};
use suma_core::optimization::linear::algorithms::simplex::solve_primal;

// Domain Imports
use crate::domains::optimization::ast::{
    OptimizationBlock, OptimizationModel, OptimizationQuery, 
    OptimizationRequest, OptimizationDirection
};
use crate::domains::queries::ast::QueryBlock;
use suma_core::symbolics::ast::Expr;
use crate::outputs::CodexOutput;

pub struct OptimizationExecutor {
    verbose: bool,
    // Memoria persistente para guardar modelos entre bloques
    models: HashMap<String, LinearProblem>,
}

impl OptimizationExecutor {
    pub fn new(verbose: bool) -> Self {
        Self { 
            verbose,
            models: HashMap::new(),
        }
    }

    pub fn execute<F>(&mut self, block: &OptimizationBlock, observer: &mut F) -> Result<()>
    where F: FnMut(&str, CodexOutput) 
    {
        match block {
            OptimizationBlock::Definition(model) => self.handle_definition(model, observer),
            OptimizationBlock::Query(query) => self.handle_query(query, observer),
        }
    }

    // --- IMPLEMENTACIÓN DEL POLIMORFISMO (Query Genérica) ---
    pub fn try_execute_query<F>(&mut self, query: &QueryBlock, observer: &mut F) -> bool
    where F: FnMut(&str, CodexOutput) 
    {
        // 1. Chequeo de existencia
        if !self.models.contains_key(&query.target_id) {
            return false;
        }

        if self.verbose { println!(">> OptAdapter: Generic Query for '{}'", query.target_id); }
        
        let problem = self.models.get(&query.target_id).unwrap();

        // 2. Iterar comandos genéricos
        for cmd in &query.commands {
            match cmd.action.as_str() {
                "solve" | "optimize" | "run" => {
                    match solve_primal(problem) {
                        Ok(solution) => {
                            let mut out = String::new();
                            // Formato Compacto: "Optimal (Z = 550.0000)"
                            writeln!(out, "{:?} (Z = {:.4})", solution.status, solution.objective_value).unwrap();
                            
                            for (k, v) in &solution.variables {
                                // Opcional: mostrar solo si > 0.0001 si quieres limpiar más
                                writeln!(out, "  {} = {:.4}", k, v).unwrap();
                            }
                            observer("Result", CodexOutput::Message(out));
                        },
                        Err(e) => observer("Error", CodexOutput::Error(format!("{}", e))),
                    }
                },
                "shadow_prices" | "sensitivity" => {
                    match solve_primal(problem) {
                        Ok(solution) => {
                            let mut out = String::new();
                            // Solo listamos valores, sin encabezado gigante
                            for (k, v) in &solution.shadow_prices {
                                writeln!(out, "  {}: {:.4}", k, v).unwrap();
                            }
                            observer("Shadow Prices", CodexOutput::Message(out));
                        },
                        Err(e) => observer("Error", CodexOutput::Error(format!("{}", e))),
                    }
                },
                "check_feasibility" => {
                     match solve_primal(problem) {
                        Ok(_) => observer("Feasibility", CodexOutput::Message("Factible".into())),
                        Err(_) => observer("Feasibility", CodexOutput::Message("Infactible".into())),
                    }
                },
                _ => {
                    observer("Warning", CodexOutput::Error(format!("Comando '{}' no soportado por Optimization", cmd.action)));
                }
            }
        }
        
        true
    }

    // --- Lógica de Definición (Guardar modelo) ---
    fn handle_definition<F>(&mut self, model: &OptimizationModel, observer: &mut F) -> Result<()>
    where F: FnMut(&str, CodexOutput) 
    {
        if self.verbose { println!(">> OptAdapter: Storing model '{}'", model.name); }

        // 1. Convertir Dirección
        let direction = match model.direction {
            OptimizationDirection::Maximize => CoreDirection::Maximize,
            OptimizationDirection::Minimize => CoreDirection::Minimize,
        };

        // 2. Linearizar Objetivo
        let obj_expr = self.linearize(&model.objective)
            .map_err(|e| anyhow!("Error en objetivo: {}", e))?;
        
        // SIN TRUCOS: Pasamos el objetivo tal cual. El Core se encarga de invertir si es necesario.
        let objective = Objective { direction, expression: obj_expr };
        let mut problem = LinearProblem::new(&model.name, objective);
        
        // 3. Procesar Restricciones
        for (i, c) in model.constraints.iter().enumerate() {
            let combined = Expr::Sub(Box::new(c.left.clone()), Box::new(c.right.clone()));
            let mut lin = self.linearize(&combined)
                .map_err(|e| anyhow!("Error en restricción {}: {}", i, e))?;
            
            let rhs = -lin.constant;
            lin.set_constant(0.0);
            
            let relation = match c.relation.as_str() {
                "<=" => Relation::LessOrEqual,
                ">=" => Relation::GreaterOrEqual,
                "=" => Relation::Equal,
                _ => return Err(anyhow!("Relación no soportada")),
            };
            
            problem.add_constraint(Constraint::new(lin, relation, rhs).with_name(&format!("c{}", i)));
        }

        // 4. GUARDAR EN EL HASHMAP
        self.models.insert(model.name.clone(), problem);
        
        if self.verbose {
            observer("System", CodexOutput::Message(format!("Modelo de optimización '{}' registrado.", model.name)));
        }
        Ok(())
    }

    // --- Lógica de Query Específica (Legacy / Internal) ---
    // Mantenemos esto por compatibilidad con el AST específico, pero con el mismo formato limpio.
    fn handle_query<F>(&mut self, query: &OptimizationQuery, observer: &mut F) -> Result<()>
    where F: FnMut(&str, CodexOutput) 
    {
        let problem = self.models.get(&query.target_id)
            .ok_or_else(|| anyhow!("Modelo '{}' no encontrado. Defínelo antes de consultarlo.", query.target_id))?;

        if self.verbose { println!(">> OptAdapter: Querying '{}'", query.target_id); }

        for req in &query.requests {
            match req {
                OptimizationRequest::Solve => {
                    let solution = solve_primal(problem).map_err(|e| anyhow!("{}", e))?;
                    
                    let mut out = String::new();
                    writeln!(out, "{:?} (Z = {:.4})", solution.status, solution.objective_value)?;
                    for (k, v) in &solution.variables {
                        writeln!(out, "  {} = {:.4}", k, v)?;
                    }
                    observer("Result", CodexOutput::Message(out));
                },
                OptimizationRequest::ShadowPrices => {
                    let solution = solve_primal(problem).map_err(|e| anyhow!("{}", e))?;
                    let mut out = String::new();
                    for (k, v) in &solution.shadow_prices {
                        writeln!(out, "  {}: {:.4}", k, v)?;
                    }
                    observer("Shadow Prices", CodexOutput::Message(out));
                },
                OptimizationRequest::CheckFeasibility => {
                     match solve_primal(problem) {
                        Ok(_) => observer("Feasibility", CodexOutput::Message("Factible".into())),
                        Err(_) => observer("Feasibility", CodexOutput::Message("Infactible".into())),
                    }
                }
            }
        }
        Ok(())
    }

    // --- Helpers de Linearización ---
    fn linearize(&self, expr: &Expr) -> Result<LinearExpression> {
        let mut lin = LinearExpression::new();
        match expr {
            Expr::Const(c) => lin.set_constant(*c),
            Expr::Var(name) => lin.add_term(name, 1.0),
            Expr::Add(lhs, rhs) => {
                let l = self.linearize(lhs)?;
                let r = self.linearize(rhs)?;
                self.merge_into(&mut lin, &l, 1.0);
                self.merge_into(&mut lin, &r, 1.0);
            },
            Expr::Sub(lhs, rhs) => {
                let l = self.linearize(lhs)?;
                let r = self.linearize(rhs)?;
                self.merge_into(&mut lin, &l, 1.0);
                self.merge_into(&mut lin, &r, -1.0);
            },
            Expr::Neg(inner) => {
                let l = self.linearize(inner)?;
                self.merge_into(&mut lin, &l, -1.0);
            },
            Expr::Mul(lhs, rhs) => {
                let left_is_const = matches!(**lhs, Expr::Const(_));
                let right_is_const = matches!(**rhs, Expr::Const(_));

                if left_is_const {
                    if let Expr::Const(c) = **lhs {
                        let r = self.linearize(rhs)?;
                        self.merge_into(&mut lin, &r, c);
                    }
                } else if right_is_const {
                     if let Expr::Const(c) = **rhs {
                        let l = self.linearize(lhs)?;
                        self.merge_into(&mut lin, &l, c);
                    }
                } else {
                    return Err(anyhow!("No linealidad: multiplicación de variables"));
                }
            },
            Expr::Div(lhs, rhs) => {
                if let Expr::Const(c) = **rhs {
                    if c.abs() < 1e-9 { return Err(anyhow!("División por cero")); }
                    let l = self.linearize(lhs)?;
                    self.merge_into(&mut lin, &l, 1.0 / c);
                } else {
                    return Err(anyhow!("División por variable no soportada"));
                }
            },
        }
        Ok(lin)
    }

    fn merge_into(&self, dest: &mut LinearExpression, src: &LinearExpression, scale: f64) {
        let new_const = dest.constant + (src.constant * scale);
        dest.set_constant(new_const);
        for (var, coeff) in &src.coefficients {
            let current = *dest.coefficients.get(var).unwrap_or(&0.0);
            dest.add_term(var, current + (coeff * scale));
        }
    }
}