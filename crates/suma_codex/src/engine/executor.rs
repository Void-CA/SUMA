use crate::ast::CodexResult;
// Importamos el adaptador
use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;
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

        for (i, result) in results.iter().enumerate() {
            if verbose {
                print!("   [{}] ", i + 1);
            }

            match result {
                CodexResult::Optimization(model) => {
                    if verbose {
                        let name = model.name.as_deref().unwrap_or("Unnamed");
                        println!("[OPTIMIZATION] Routing model: '{}'", name);
                    }
                    // TODO: opt_adapter.execute(model, &mut observer)...
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
                    
                    // CRÍTICO: Pasamos '&mut observer'.
                    // Como el adaptador espera un genérico F, al pasarle una referencia mutable,
                    // Rust infiere que el tipo genérico del adaptador es &mut F.
                    // Esto permite reutilizar el closure en la siguiente iteración del loop.
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
}