use std::collections::HashMap;
use suma_core::linear_algebra::{DenseMatrix, systems::LinearSystem};
use crate::domains::linear_algebra::ast::{LinearAlgebraModel, LinearAlgebraAction, SystemDef};
use colored::*;

pub struct LinearAlgebraExecutor {
    verbose: bool,
    // MEMORIA: Aquí persisten los artefactos entre llamadas
    artifacts: HashMap<String, SystemDef>,
}

impl LinearAlgebraExecutor {
    pub fn new(verbose: bool) -> Self {
        Self { 
            verbose,
            artifacts: HashMap::new(),
        }
    }

    pub fn execute(&mut self, model: &LinearAlgebraModel) -> Result<(), String> {
        if self.verbose {
            println!("   -> Processing Block: {}", model.name.as_deref().unwrap_or("Unnamed"));
        }

        // Iteramos sobre las acciones dentro del bloque
        for action in &model.actions {
            match action {
                // 1. DEFINICIÓN: Guardar Artefacto
                LinearAlgebraAction::System(sys) => {
                    if self.verbose { println!("      [DEFINE] Artifact '{}'", sys.name); }
                    self.artifacts.insert(sys.name.clone(), sys.clone());
                },

                // 2. ANÁLISIS: Ejecutar Query
                LinearAlgebraAction::Analysis(query) => {
                    if self.verbose { println!("      [QUERY] Analyzing target '{}'", query.target); }
                    
                    // A. Recuperar Artefacto
                    let system = self.artifacts.get(&query.target)
                        .ok_or_else(|| format!("Artifact '{}' not found. Define it first using 'LinearSystem'.", query.target))?;

                    // B. Cargar Datos
                    let a = if let Some(d) = &system.matrix_a {
                        DenseMatrix::new(d.rows, d.cols, d.data.clone())
                    } else {
                        return Err(format!("Artifact '{}' is missing coefficients.", system.name));
                    };

                    println!("Results for {}:", query.name.bold());

                    // C. Calcular Propiedades Solicitadas
                    for req in &query.calculate {
                        match req.as_str() {
                            "determinant" => {
                                match a.determinant() {
                                    Ok(val) => println!(" - Determinant: {:.4}", val),
                                    Err(_) => println!(" - Determinant: N/A"),
                                }
                            },
                            "solution" => {
                                if let Some(db) = &system.vector_b {
                                    let b = DenseMatrix::new(db.rows, db.cols, db.data.clone());
                                    match LinearSystem::solve(&a, &b) {
                                        Ok(x) => println!(" - Solution:\n{}", x),
                                        Err(e) => println!(" - Solution: Error ({})", e),
                                    }
                                } else {
                                    println!(" - Solution: Cannot solve (missing constants 'b')");
                                }
                            },
                            _ => println!(" - {}: Unknown property", req),
                        }
                    }
                }
            }
        }
        Ok(())
    }
}