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
        // Esto permite que el HashMap de artefactos persista entre un bloque 'LinearSystem'
        // y un bloque 'Analysis' subsiguiente.
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

                    // Usamos la instancia persistente del adaptador
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
        // El registro ahora es automático basado en valid_keywords()
        engine.register(OptimizationParser);
        engine.register(BooleanParser);
        engine.register(LinearAlgebraParser);
        engine
    }

    // TEST 1: Ejecución Declarativa Real
    // Probamos la definición y el análisis en bloques separados pero secuenciales
    #[test]
    fn test_linear_algebra_execution_declarative() {
        let engine = engine_setup();
        
        // NOTA: Ya no usamos 'linear_algebra "Math" { ... }'.
        // Usamos directamente los keywords del dominio.
        let code = r#"
        LinearSystem "Sistema_1" {
            coefficients: [1, 2; 3, 4]
            constants:    [5; 6]
        }
        
        Analysis "Consulta" {
            target: "Sistema_1"
            calculate: [ determinant, solution ]
        }
        "#;

        println!("\n--- TEST: EJECUCIÓN DECLARATIVA ---");
        let results = engine.process_file(code);
        
        // Ahora esperamos 2 bloques independientes: 1 System + 1 Analysis
        assert_eq!(results.len(), 2, "Debería haber 2 bloques procesados");

        // Ejecutamos con verbose=true para ver los logs en el test
        CodexExecutor::execute(results, true);
    }

    // TEST 2: Persistencia entre bloques explícita
    #[test]
    fn test_cross_block_persistence() {
        let engine = engine_setup();
        let code = r#"
        LinearSystem "GlobalSys" {
            coefficients: [1, 0; 0, 1]
        }

        Analysis "Check" {
            target: "GlobalSys"
            calculate: [ determinant ]
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
        Analysis "Busqueda_Fantasma" {
            target: "Sistema_Inexistente"
            calculate: [ determinant ]
        }
        "#;

        println!("\n--- TEST: ARTEFACTO NO ENCONTRADO ---");
        let results = engine.process_file(code);
        
        assert_eq!(results.len(), 1);
        // Esto imprimirá un error en consola "[ERROR] Runtime failure..." pero no debe hacer panic
        CodexExecutor::execute(results, true);
    }

    // TEST 4: Routing Multiparadigma
    // Verificamos que keywords de distintos dominios convivan en el mismo archivo
    #[test]
    fn test_mixed_domains_routing() {
        let engine = engine_setup();

        let code = r#"
        // 2. Dominio Álgebra Lineal
        LinearSystem "Sys" { 
            coefficients: [2, 0; 0, 2] 
        }
        
        Analysis "Det" { 
            target: "Sys"
            calculate: [determinant] 
        }

        "#;

        println!("\n--- TEST: ROUTING MULTIPARADIGMA ---");
        let results = engine.process_file(code);

        // Esperamos 2 bloques: Sys, Analysis
        assert_eq!(results.len(), 2, "El parser debe identificar 2 bloques distintos");
        
        // Validar orden y tipos
        if !matches!(results[0], CodexResult::LinearAlgebra(_)) { panic!("Bloque 1 debe ser LinearAlgebra (System)"); }
        if !matches!(results[1], CodexResult::LinearAlgebra(_)) { panic!("Bloque 2 debe ser Analysis"); }

        CodexExecutor::execute(results, true);
    }
}