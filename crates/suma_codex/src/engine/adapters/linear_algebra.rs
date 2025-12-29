use std::collections::HashMap;
use colored::*; // Necesario solo para tus logs internos de verbose

use suma_core::linear_algebra::{DenseMatrix, systems::LinearSystem};
use crate::domains::linear_algebra::ast::{LinearAlgebraBlock, LinAlgStmt, SystemDef, QueryDef, Capability};

// Importamos el contrato de salida
use crate::outputs::CodexOutput;

pub struct LinearAlgebraExecutor {
    verbose: bool,
    artifacts: HashMap<String, SystemDef>,
}

impl LinearAlgebraExecutor {
    pub fn new(verbose: bool) -> Self {
        Self { verbose, artifacts: HashMap::new() }
    }

    // Punto de entrada principal
    pub fn execute<F>(&mut self, block: &LinearAlgebraBlock, mut observer: F) -> Result<(), String> 
    where F: FnMut(&str, CodexOutput) 
    {
        for stmt in &block.statements {
            match stmt {
                LinAlgStmt::System(def) => {
                    self.register_system(def)?;
                    // Notificamos al CLI que se guardó (opcional, como mensaje de bajo nivel)
                    if self.verbose {
                        observer(&def.id, CodexOutput::Message("Modelo definido en memoria.".into()));
                    }
                },
                LinAlgStmt::Query(query) => {
                    // Pasamos la referencia mutable del observer
                    self.execute_query(query, &mut observer)?;
                }
            }
        }
        Ok(())
    }

    // --- Lógica Interna ---

    fn register_system(&mut self, def: &SystemDef) -> Result<(), String> {
        if self.verbose {
            println!("      [DEFINE] Guardando modelo '{}'", def.id.cyan());
        }
        self.artifacts.insert(def.id.clone(), def.clone());
        Ok(())
    }

    // Fusión de lógica: Acepta el observer y usa CodexOutput
    fn execute_query<F>(&self, query: &QueryDef, observer: &mut F) -> Result<(), String> 
    where F: FnMut(&str, CodexOutput)
    {
        if self.verbose {
            println!("      [QUERY] Consultando modelo '{}'", query.target_id.cyan());
        }

        // 1. HIDRATACIÓN (Buscar y convertir a Core)
        let system = self.artifacts.get(&query.target_id)
            .ok_or_else(|| format!(
                "El modelo '{}' no existe. Defínelo antes con 'LinearSystem'.", 
                query.target_id
            ))?;

        // Recuperar Matriz A
        let matrix_a = if let Some(data) = &system.coefficients {
            DenseMatrix::new(data.rows, data.cols, data.data.clone())
        } else {
            return Err(format!("El modelo '{}' no tiene coeficientes (A).", system.id));
        };

        // 2. EJECUCIÓN DE CAPABILITIES
        for req in &query.requests {
            // Generar etiqueta (Alias o default)
            let temp_label = format!("{:?}", req.capability);
            let label = req.alias.as_ref().unwrap_or(&temp_label);

            match req.capability {
                Capability::Determinant => {
                    match matrix_a.determinant() {
                        Ok(val) => {
                            // ÉXITO: Enviamos Scalar
                            observer(label, CodexOutput::LinAlgScalar(val));
                        },
                        Err(e) => {
                            // FALLO MATEMÁTICO: Enviamos Error
                            observer(label, CodexOutput::Error(format!("Determinante indefinido: {}", e)));
                        }
                    }
                },
                Capability::Solution => {
                    if let Some(data_b) = &system.constants {
                        let vector_b = DenseMatrix::new(data_b.rows, data_b.cols, data_b.data.clone());
                        
                        match LinearSystem::solve(&matrix_a, &vector_b) {
                            Ok(result_vec) => {
                                // ÉXITO: Enviamos Vector
                                observer(label, CodexOutput::LinAlgVector(result_vec));
                            },
                            Err(e) => {
                                observer(label, CodexOutput::Error(format!("Sin solución: {}", e)));
                            }
                        }
                    } else {
                        observer(label, CodexOutput::Error("Faltan constantes 'b' para resolver.".into()));
                    }
                },
                Capability::Inverse => {
                    // Placeholder para futura implementación
                    // Aquí usarías CodexOutput::LinAlgMatrix(res)
                    observer(label, CodexOutput::Message("Cálculo de Inversa no implementado aún.".into()));
                },
                Capability::Rank | Capability::Trace => {
                    observer(label, CodexOutput::Message("Función pendiente en Core.".into()));
                }
            }
        }
        Ok(())
    }
}