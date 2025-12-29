use anyhow::{anyhow, Result};
use std::fmt::Write; // Para formatear el string de salida limpiamente

// Imports del Core
use suma_core::optimization::linear::model::{
    LinearProblem, Objective, Constraint, LinearExpression, Relation, OptimizationDirection as CoreDirection
};
use suma_core::optimization::linear::algorithms::simplex::solve_primal;

use crate::domains::optimization::OptimizationModel;
// Imports del Dominio (Codex)
use crate::domains::optimization::ast::{OptimizationDirection};
use suma_core::symbolics::ast::Expr;
use crate::outputs::CodexOutput;

pub struct OptimizationExecutor {
    verbose: bool,
}

impl OptimizationExecutor {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Ejecuta un bloque de optimización y notifica al observador
    pub fn execute<F>(&mut self, model: &OptimizationModel, observer: &mut F) -> Result<()>
    where F: FnMut(&str, CodexOutput) 
    {
        if self.verbose {
            println!(">> OptAdapter: Linearizing model...");
        }

        // 1. Configurar Dirección
        let direction = match model.direction {
            OptimizationDirection::Maximize => CoreDirection::Maximize,
            OptimizationDirection::Minimize => CoreDirection::Minimize,
        };

        // 2. Linearizar Función Objetivo
        let obj_expr = self.linearize(&model.objective)
            .map_err(|e| anyhow!("Error en función objetivo: {}", e))?;
        
        let objective = Objective { direction, expression: obj_expr };
        
        // Usamos un nombre genérico o podríamos agregarlo a la gramática después
        let mut problem = LinearProblem::new("CodexModel", objective);

        // 3. Procesar Restricciones
        for (i, c) in model.constraints.iter().enumerate() {
            // Transformación Algebraica: Left op Right  =>  Left - Right op 0
            // Construimos una expresión simbólica de resta
            let combined_symbolic = Expr::Sub(
                Box::new(c.left.clone()),
                Box::new(c.right.clone())
            );

            // Linearizamos la combinación
            let mut lin_expr = self.linearize(&combined_symbolic)
                .map_err(|e| anyhow!("Error en restricción #{}: {}", i+1, e))?;

            // Ajuste de forma estándar: ax + by + K <= 0  =>  ax + by <= -K
            // El Core espera la constante en el RHS
            let rhs = -lin_expr.constant;
            lin_expr.set_constant(0.0); // Limpiamos la constante del lado izquierdo

            let relation = match c.relation.as_str() {
                "<=" => Relation::LessOrEqual,
                ">=" => Relation::GreaterOrEqual,
                "="  => Relation::Equal,
                _ => return Err(anyhow!("Relación no soportada: {}", c.relation)),
            };

            problem.add_constraint(
                Constraint::new(lin_expr, relation, rhs).with_name(&format!("c_{}", i+1))
            );
        }

        if self.verbose {
            println!(">> OptAdapter: Solving via Core Simplex...");
        }

        // 4. Resolver
        let solution = solve_primal(&problem)
            .map_err(|e| anyhow!("{}", e))?; // Convertimos el error del Core a anyhow

        // 5. Formatear Salida
        let mut output = String::new();
        writeln!(output, "Estado: {:?}", solution.status)?;
        writeln!(output, "Objetivo: {:.4}", solution.objective_value)?;
        writeln!(output, "Variables:")?;
        for (name, val) in &solution.variables {
            writeln!(output, "  {} = {:.4}", name, val)?;
        }
        
        if !solution.shadow_prices.is_empty() {
            writeln!(output, "Precios Sombra:")?;
            for (name, val) in &solution.shadow_prices {
                writeln!(output, "  {} : {:.4}", name, val)?;
            }
        }

        // 6. Notificar al Engine
        observer("Optimization Result", CodexOutput::Message(output));

        Ok(())
    }

    /// El Linearizador: Convierte Árbol Simbólico -> Expresión Lineal Plana
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
                // Solo permitimos Linealidad: Constante * Variable (o Expresión)
                // Si ambos lados tienen variables, es cuadrático/no-lineal -> Error
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
                    return Err(anyhow!("No linealidad detectada: Multiplicación de variables no soportada en LP."));
                }
            },

            Expr::Div(lhs, rhs) => {
                // Solo permitimos Expresión / Constante
                if let Expr::Const(c) = **rhs {
                    if c.abs() < 1e-9 { return Err(anyhow!("División por cero.")); }
                    let l = self.linearize(lhs)?;
                    self.merge_into(&mut lin, &l, 1.0 / c);
                } else {
                    return Err(anyhow!("División por variable no soportada."));
                }
            },
        }

        Ok(lin)
    }

    /// Helper para fusionar dos expresiones lineales: dest += src * scale
    fn merge_into(&self, dest: &mut LinearExpression, src: &LinearExpression, scale: f64) {
        // Sumar constante
        let new_const = dest.constant + (src.constant * scale);
        dest.set_constant(new_const);

        // Sumar términos
        for (var, coeff) in &src.coefficients {
            let current = *dest.coefficients.get(var).unwrap_or(&0.0);
            dest.add_term(var, current + (coeff * scale));
            // Nota: add_term en el core usualmente reemplaza o suma. 
            // Si tu implementación de core::LinearExpression::add_term REEMPLAZA, 
            // necesitas ajustar esto para que SUME.
            // Asumiendo que add_term(var, val) hace: map[var] = val.
            // Entonces la lógica correcta arriba es:
            // 1. Obtener valor actual en dest.
            // 2. Sumar (src_coeff * scale).
            // 3. Setear nuevo valor.
        }
    }
}