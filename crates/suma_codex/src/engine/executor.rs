// src/engine/executor.rs
use crate::ast::CodexResult;
use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;

pub struct CodexExecutor;

impl CodexExecutor {
    pub fn execute(results: Vec<CodexResult>, verbose: bool) {
        if verbose {
            println!(">> Executor: Processing {} blocks...", results.len());
        }

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
                        println!("[LINEAR ALGEBRA] Processing model: '{}'", name);
                    }

                    let mut adapter = LinearAlgebraExecutor::new(verbose);

                    if let Err(e) = adapter.execute(model) {
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

    // TEST 1: Verificar que Linear Algebra funciona realmente (Cálculo real)
    #[test]
    fn test_linear_algebra_execution_real() {
        let engine = engine_setup();
        let code = r#"
        linear_algebra "Math_Real" {
            let A = [1, 2; 3, 4]
            let B = [1, 0; 0, 1]
            let C = A * B        // Debe dar A
        }
        "#;

        println!("\n--- TEST: EJECUCIÓN REAL (LINEAR ALG) ---");
        let results = engine.process_file(code);
        assert_eq!(results.len(), 1);

        // Esto debe imprimir la matriz resultado en consola
        CodexExecutor::execute(results);
    }

    // TEST 2: Verificar Aislamiento de Scope (Linear Algebra)
    #[test]
    fn test_scope_isolation() {
        let engine = engine_setup();
        let code = r#"
        linear_algebra "Contexto_A" { let x = [10] }
        linear_algebra "Contexto_B" { let y = x } // Fallará, 'x' no existe aquí
        "#;

        println!("\n--- TEST: SCOPE ISOLATION ---");
        let results = engine.process_file(code);

        // Esperamos ver un error en logs para el segundo bloque, pero no un panic del test
        CodexExecutor::execute(results);
    }

    // TEST 3: Routing Multiparadigma (El más importante para tu solicitud)
    // Verifica que el Executor maneja la mezcla de dominios funcionales y stubs.
    #[test]
    fn test_mixed_domains_routing() {
        let engine = engine_setup();

        // Código que mezcla los 3 lenguajes
        let code = r#"
        // 1. Stub
        optimize "Problema_Produccion" {
            maximize: profit
            constraints: profit > 100
        }

        // 2. Real
        linear_algebra "Calculo_Matrices" {
            let M = [5, 5; 2, 2]
            let det = det(M) // Debería dar 0
        }

        // 3. Stub
        boolean "Circuito_Logico" {
            (A and B) or (not C)
        }
        "#;

        println!("\n--- TEST: ROUTING MULTIPARADIGMA ---");
        let results = engine.process_file(code);

        // 1. Validar Parsing (Nivel Codex)
        assert_eq!(results.len(), 3, "El parser debe separar los 3 bloques correctamente");

        // Validar Tipos en orden
        if !matches!(results[0], CodexResult::Optimization(_)) { panic!("Bloque 1 debe ser Optimization"); }
        if !matches!(results[1], CodexResult::LinearAlgebra(_)) { panic!("Bloque 2 debe ser LinearAlgebra"); }
        if !matches!(results[2], CodexResult::Boolean(_)) { panic!("Bloque 3 debe ser Boolean"); }

        // 2. Validar Ejecución (Nivel Engine)
        // Esto imprimirá mensajes de "Aviso: Pendiente" para 1 y 3, y ejecutará el 2.
        CodexExecutor::execute(results);
    }
}