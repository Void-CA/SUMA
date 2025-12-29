use crate::ast::CodexResult;
// Importamos el adaptador
use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;
use crate::engine::adapters::optimization::OptimizationExecutor;
// Importamos el contrato de salida
use crate::outputs::CodexOutput;

pub struct CodexExecutor;

impl CodexExecutor {
    /// Ejecuta una lista de resultados (Bloques parseados).
    /// 
    /// # Argumentos
    /// * `results` - Vector de artefactos/queries parseados.
    /// * `verbose` - Flag para logs internos de debug.
    /// * `observer` - Callback (Closure) que maneja la salida visual.
    pub fn execute<F>(results: Vec<CodexResult>, verbose: bool, mut observer: F) 
    where F: FnMut(&str, CodexOutput) 
    {
        if verbose {
            println!(">> Executor: Orchestrating {} blocks...", results.len());
        }

        // --- PERSISTENCIA DE ESTADO ---
        // Instanciamos el adaptador FUERA del loop para mantener la memoria
        // (artifacts) entre bloques secuenciales.
        let mut lin_alg_adapter = LinearAlgebraExecutor::new(verbose);
        let mut opt_adapter = OptimizationExecutor::new(verbose);

        for (i, result) in results.iter().enumerate() {
            if verbose {
                print!("   [{}] ", i + 1);
            }

            match result {
                CodexResult::Optimization(block) => {
                    if verbose { println!("[OPTIMIZATION] Processing block"); }
                    
                    // Ejecutamos pasando el observer
                    if let Err(e) = opt_adapter.execute(block, &mut observer) {
                         observer("Optimization Error", CodexOutput::Error(format!("{}", e)));
                    }
                },

                CodexResult::Boolean(model) => {
                    if verbose {
                        let name = model.name.as_deref().unwrap_or("Unnamed");
                        println!("[BOOLEAN] Routing model: '{}'", name);
                    }
                    // TODO: bool_adapter.execute(model, &mut observer)...
                },

                CodexResult::LinearAlgebra(block) => {
                    if verbose {
                        println!("[LINEAR ALGEBRA] Processing block");
                    }
                    
                    if let Err(e) = lin_alg_adapter.execute(block, &mut observer) {
                        observer("Runtime Error", CodexOutput::Error(e));
                    }
                },
            }
        }
    }
}

// ==========================================
// TESTS DE INTEGRACIÓN
// ==========================================
#[cfg(test)]
mod tests {
    use crate::CodexEngine;
    use super::*;

    // Imports de Parsers
    use crate::domains::optimization::OptimizationParser;
    use crate::domains::boolean_algebra::BooleanParser;
    use crate::domains::linear_algebra::parser::LinearAlgebraParser;

    fn engine_setup() -> CodexEngine {
        let mut engine = CodexEngine::new();
        engine.register(OptimizationParser);
        engine.register(BooleanParser);
        engine.register(LinearAlgebraParser);
        engine
    }

    // Helper para simular un CLI en los tests
    fn test_observer(alias: &str, output: CodexOutput) {
        println!("[TEST OUTPUT] {}: {:?}", alias, output);
    }

    // TEST 1: Ejecución Declarativa Real
    #[test]
    fn test_linear_algebra_execution_declarative() {
        let engine = engine_setup();
        
        let code = r#"
        LinearSystem "Sistema_1" {
            coefficients: [1, 2; 3, 4]
            constants:    [5; 6]
        }
        
        query "Sistema_1" {
            determinant as det_val
            solution    as sol_vec
        }
        "#;

        println!("\n--- TEST: EJECUCIÓN DECLARATIVA ---");
        let results = engine.process_file(code);
        
        // CORRECCIÓN: Ahora pasamos el closure (observer)
        CodexExecutor::execute(results, true, |alias, output| {
            test_observer(alias, output);
        });
    }

    // TEST 2: Persistencia y Sugar
    #[test]
    fn test_cross_block_persistence() {
        let engine = engine_setup();
        let code = r#"
        LinearSystem "GlobalSys" {
            coefficients: [1, 0; 0, 1]
            constants:    [10; 20]
        }

        query "GlobalSys" {
            solution
        }
        "#;

        println!("\n--- TEST: PERSISTENCIA ENTRE BLOQUES ---");
        let results = engine.process_file(code);
        
        // CORRECCIÓN: Pasamos el closure
        CodexExecutor::execute(results, true, |alias, output| {
            test_observer(alias, output);
        });
    }

    // TEST 3: Manejo de Errores
    #[test]
    fn test_missing_artifact_error() {
        let engine = engine_setup();
        let code = r#"
        query "Sistema_Fantasma" {
            determinant
        }
        "#;

        println!("\n--- TEST: ARTEFACTO NO ENCONTRADO ---");
        let results = engine.process_file(code);
        
        // CORRECCIÓN: Pasamos el closure y verificamos visualmente que salga el error
        CodexExecutor::execute(results, true, |alias, output| {
            if let CodexOutput::Error(msg) = output {
                println!("[TEST OK] Error capturado correctamente: {}: {}", alias, msg);
            } else {
                test_observer(alias, output);
            }
        });
    }

    #[test]
    fn test_optimization_pipeline_full() {
        let mut engine = engine_setup(); // Tu setup que registra parsers
        
        // Código Codex de alto nivel
        let code = r#"
        optimization {
            maximize 3*x + 5*y
            constraints {
                x + 2*y <= 20
                x <= 10
            }
        }
        "#;

        println!("\n--- TEST: PIPELINE DE OPTIMIZACIÓN ---");
        let results = engine.process_file(code);
        
        CodexExecutor::execute(results, true, |alias, output| {
            println!("[TEST OUT] {}: {:?}", alias, output);
            
            if let CodexOutput::Message(txt) = output {
                // Verificamos que contenga la solución correcta (550)
                assert!(txt.contains("550"), "El output debe contener el óptimo 550");
                assert!(txt.contains("Variables"), "Debe listar variables");
            } else {
                panic!("Se esperaba salida de texto, se obtuvo: {:?}", output);
            }
        });
    }
}