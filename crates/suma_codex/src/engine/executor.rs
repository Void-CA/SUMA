use crate::ast::CodexResult;
use crate::outputs::CodexOutput;

// Importamos los adaptadores
// Asegúrate de que estos módulos sean pub en 'src/engine/adapters/mod.rs'
use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;
use crate::engine::adapters::optimization::OptimizationExecutor;

pub struct CodexExecutor;

impl CodexExecutor {
    /// Ejecuta una lista de resultados (Bloques parseados).
    /// 
    /// # Arquitectura
    /// Instancia los adaptadores con memoria (Stateful) antes del bucle.
    /// Itera sobre los resultados y despacha según el tipo.
    /// Para las Queries, utiliza un patrón de "Cadena de Responsabilidad".
    pub fn execute<F>(results: Vec<CodexResult>, verbose: bool, mut observer: F) 
    where F: FnMut(&str, CodexOutput) 
    {
        if verbose {
            println!(">> Executor: Orchestrating {} blocks...", results.len());
        }

        // --- 1. PERSISTENCIA DE ESTADO ---
        // Instanciamos los adaptadores FUERA del loop.
        // Esto permite que una definición en el paso 1 sea recordada en el paso 5.
        let mut lin_alg = LinearAlgebraExecutor::new(verbose);
        let mut opt = OptimizationExecutor::new(verbose);
        // let mut bool_exec = BooleanExecutor::new(verbose); 

        // --- 2. BUCLE DE EJECUCIÓN ---
        for (_i, result) in results.iter().enumerate() {
            if verbose {
                // print!("   [{}] ", i + 1); // Opcional: log de paso
            }

            match result {
                // --- DEFINICIONES DE DOMINIO ---
                
                CodexResult::LinearAlgebra(block) => {
                    if verbose { println!("[LINEAR ALGEBRA] Processing definition"); }
                    // Pasamos referencia &block
                    if let Err(e) = lin_alg.execute(block, &mut observer) {
                        observer("Runtime Error", CodexOutput::Error(format!("{}", e)));
                    }
                },

                CodexResult::Optimization(block) => {
                    if verbose { println!("[OPTIMIZATION] Processing definition"); }
                    // Pasamos referencia &block
                    if let Err(e) = opt.execute(block, &mut observer) {
                        observer("Optimization Error", CodexOutput::Error(format!("{}", e)));
                    }
                },

                CodexResult::Boolean(model) => {
                    if verbose { println!("[BOOLEAN] Processing definition: {:?}", model.name); }
                    // Placeholder hasta que tengas el BooleanExecutor listo
                    observer("System", CodexOutput::Message("Dominio Booleano registrado (Sin ejecución aún)".into()));
                },

                // --- QUERY GENÉRICA (POLIMORFISMO) ---
                
                CodexResult::Query(query) => {
                    if verbose { println!("[QUERY] Broadcasting query for '{}'", query.target_id); }

                    // Estrategia "Broadcast" / "Chain of Responsibility"
                    // Le preguntamos a cada adaptador si reconoce el ID.
                    
                    // 1. Preguntar a Álgebra Lineal
                    let handled_lin = lin_alg.try_execute_query(query, &mut observer);
                    
                    // 2. Preguntar a Optimización (solo si no fue atendido)
                    let handled_opt = if !handled_lin {
                        opt.try_execute_query(query, &mut observer)
                    } else {
                        true 
                    };

                    // 3. Si nadie respondió
                    if !handled_lin && !handled_opt {
                        observer("Error", CodexOutput::Error(
                            format!("El identificador '{}' no fue encontrado en ningún dominio activo (LinearAlgebra, Optimization).", query.target_id)
                        ));
                    }
                }
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

    // Imports de Parsers (Para el setup del test)
    use crate::domains::optimization::parser::OptimizationParser;
    use crate::domains::boolean_algebra::BooleanParser;
    use crate::domains::linear_algebra::parser::LinearAlgebraParser;
    // Importante: Importar el parser de Queries globales
    use crate::domains::queries::parser::QueryParser;

    fn engine_setup() -> CodexEngine {
        let mut engine = CodexEngine::new();
        engine.register(OptimizationParser);
        engine.register(BooleanParser);
        engine.register(LinearAlgebraParser);
        engine.register(QueryParser); // <--- ¡No olvidar registrar este!
        engine
    }

    // Helper para simular un CLI en los tests
    fn test_observer(alias: &str, output: CodexOutput) {
        println!("[TEST OUTPUT] {}: {:?}", alias, output);
    }

    #[test]
    fn test_linear_algebra_execution() {
        let engine = engine_setup();
        
        // Nótese que usamos 'query' (genérico)
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

        println!("\n--- TEST: LINEAR ALGEBRA FLOW ---");
        let results = engine.process_file(code);
        
        CodexExecutor::execute(results, true, |alias, output| {
            test_observer(alias, output);
        });
    }

    #[test]
    fn test_optimization_pipeline_full() {
        let engine = engine_setup();
        
        // CORRECCIÓN: Usamos 'query', no 'run'
        let code = r#"
        Optimization "Maximizar_Producción" {
            maximize 30*x + 50*y
            constraints {
                x + 2*y <= 20
                x <= 10
            }
        }

        query "Maximizar_Producción" {
            solve
        }
        "#;

        println!("\n--- TEST: PIPELINE DE OPTIMIZACIÓN ---");
        let results = engine.process_file(code);
        
        // Bandera para evitar falsos positivos
        let mut solved = false;

        CodexExecutor::execute(results, true, |alias, output| {
            println!("[TEST OUT] {}: {:?}", alias, output);
            
            let txt = match output {
                CodexOutput::Message(s) => s,
                CodexOutput::Error(e) => panic!("Error inesperado: {}", e),
                _ => String::new(),
            };

            if alias == "System" {
                assert!(txt.contains("registrado"), "Falló la definición");
            }
            
            if alias == "Solve Result" {
                solved = true; // ¡Marcamos que pasamos por aquí!
                assert!(txt.contains("550"), "El óptimo debe ser 550. Recibido: \n{}", txt);
                assert!(txt.contains("x") && txt.contains("10"), "x debería ser 10");
            }
        });

        assert!(solved, "El test terminó sin resolver el problema (nunca recibió 'Solve Result')");
    }

    #[test]
    fn test_missing_artifact_error() {
        let engine = engine_setup();
        let code = r#"
        query "Sistema_Fantasma" {
            solve
        }
        "#;

        println!("\n--- TEST: ARTEFACTO NO ENCONTRADO ---");
        let results = engine.process_file(code);
        
        let mut error_caught = false;
        CodexExecutor::execute(results, true, |alias, output| {
            if let CodexOutput::Error(msg) = output {
                println!("[TEST OK] Error capturado correctamente: {}: {}", alias, msg);
                error_caught = true;
            }
        });
        assert!(error_caught, "El executor debería haber reportado un error de 'no encontrado'");
    }
}