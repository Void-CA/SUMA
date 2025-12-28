use crate::ast::CodexResult;
// Importamos el adaptador
use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;

pub struct CodexExecutor;

impl CodexExecutor {
    // Agregamos el flag 'verbose'
    pub fn execute(results: Vec<CodexResult>, verbose: bool) {
        if verbose {
            println!(">> Executor: Orchestrating {} blocks...", results.len());
        }

        // --- CAMBIO CRÍTICO: PERSISTENCIA ---
        // Instanciamos el adaptador FUERA del loop.
        // Esto permite que un bloque defina un sistema y el siguiente lo analice.
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
                    // Aquí iría: opt_adapter.execute(model)...
                },

                CodexResult::Boolean(model) => {
                    if verbose {
                        let name = model.name.as_deref().unwrap_or("Unnamed");
                        println!("[BOOLEAN] Routing model: '{}'", name);
                    }
                },

                CodexResult::LinearAlgebra(model) => {
                    if verbose {
                        let name = model.name.as_deref().unwrap_or("Unnamed");
                        println!("[LINEAR ALGEBRA] Processing block: '{}'", name);
                    }

                    // Usamos la instancia persistente
                    if let Err(e) = lin_alg_adapter.execute(model) {
                        eprintln!("   [ERROR] Runtime failure: {}", e);
                    }
                }
            }
        }
    }
}

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

    // TEST 1: Ejecución Declarativa Real
    #[test]
    fn test_linear_algebra_execution_declarative() {
        let engine = engine_setup();
        // Nueva Sintaxis: Definición + Análisis
        let code = r#"
        linear_algebra "Math_Real" {
            // 1. Artefacto
            LinearSystem "Sistema_1" {
                coefficients: [1, 2; 3, 4]
                constants:    [5; 6]
            }
            
            // 2. Query
            Analysis "Consulta" {
                target: "Sistema_1"
                calculate: [ determinant, solution ]
            }
        }
        "#;

        println!("\n--- TEST: EJECUCIÓN DECLARATIVA ---");
        let results = engine.process_file(code);
        assert_eq!(results.len(), 1, "Debería haber 1 bloque linear_algebra (que contiene acciones internas)");

        // Ejecutamos con verbose=true para ver los logs en el test
        CodexExecutor::execute(results, true);
    }

    // TEST 2: Persistencia entre bloques
    // Probamos que si defino un sistema en un bloque, puedo consultarlo en otro
    #[test]
    fn test_cross_block_persistence() {
        let engine = engine_setup();
        let code = r#"
        linear_algebra "Definicion" {
            LinearSystem "GlobalSys" {
                coefficients: [1, 0; 0, 1]
            }
        }

        linear_algebra "Consulta" {
            Analysis "Check" {
                target: "GlobalSys"
                calculate: [ determinant ]
            }
        }
        "#;

        println!("\n--- TEST: PERSISTENCIA ENTRE BLOQUES ---");
        let results = engine.process_file(code);
        assert_eq!(results.len(), 2);

        CodexExecutor::execute(results, true);
    }

    // TEST 3: Manejo de Errores (Artefacto no encontrado)
    #[test]
    fn test_missing_artifact_error() {
        let engine = engine_setup();
        let code = r#"
        linear_algebra "Fallo" {
            Analysis "Busqueda_Fantasma" {
                target: "Sistema_Inexistente"
                calculate: [ determinant ]
            }
        }
        "#;

        println!("\n--- TEST: ARTEFACTO NO ENCONTRADO ---");
        let results = engine.process_file(code);
        
        // Esto imprimirá un error en consola pero no debe hacer panic
        CodexExecutor::execute(results, true);
    }

    // TEST 4: Routing Multiparadigma
    #[test]
    fn test_mixed_domains_routing() {
        let engine = engine_setup();

        let code = r#"
        // 1. Stub Optimización
        optimize "Problema_Produccion" {
            maximize: profit
            constraints: profit > 100
        }

        // 2. Real Álgebra Lineal (Nueva Sintaxis)
        linear_algebra "Calculo_Matrices" {
            LinearSystem "Sys" { coefficients: [2, 0; 0, 2] }
            Analysis "Det" { target: "Sys", calculate: [determinant] }
        }

        // 3. Stub Booleano
        boolean "Circuito_Logico" {
            (A and B) or (not C)
        }
        "#;

        println!("\n--- TEST: ROUTING MULTIPARADIGMA ---");
        let results = engine.process_file(code);

        assert_eq!(results.len(), 3);
        
        // Validar orden
        if !matches!(results[0], CodexResult::Optimization(_)) { panic!("Error orden 1"); }
        if !matches!(results[1], CodexResult::LinearAlgebra(_)) { panic!("Error orden 2"); }
        if !matches!(results[2], CodexResult::Boolean(_)) { panic!("Error orden 3"); }

        CodexExecutor::execute(results, true);
    }
}