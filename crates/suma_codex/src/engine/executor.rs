// src/engine/executor.rs
use crate::ast::CodexResult;

// Aquí importamos el núcleo real. 
// Asumiremos que suma_core tiene estas funciones expuestas.
// use suma_core::optimization::Solver;
// use suma_core::boolean::Evaluator;

pub struct CodexExecutor;

impl CodexExecutor {
    pub fn execute(results: Vec<CodexResult>) {
        println!(">> Ejecutor: Procesando {} bloques de instrucciones...", results.len());

        for result in results {
            match result {
                CodexResult::Optimization(model) => {
                    println!("   [Optimización] Modelo detectado: {}", 
                        model.name.as_deref().unwrap_or("Unnamed Model"));
                    println!("   -> Enviando a suma_core::optimization...");
                    // let solucion = Solver::solve(model);
                    // println!("   Resultado: {:?}", solucion);
                },
                CodexResult::Boolean(model) => {
                    println!("   [Álgebra Booleana] Expresión detectada.");
                    println!("   -> Enviando a suma_core::boolean...");
                    // let evaluacion = Evaluator::eval(model);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodexEngine;
    use super::*;
    // Asegúrate de importar tus estructuras de dominio
    use crate::domains::optimization::OptimizationParser;
    use crate::domains::boolean_algebra::{BooleanModel, BooleanParser};

    fn engine_setup() -> CodexEngine {
        let mut engine = CodexEngine::new();
        engine.register(OptimizationParser);
        engine.register(BooleanParser);
        engine
    }

    #[test]
    fn test_executor_routing() {
        // 1. Preparar el entorno (Parser + Dispatcher)
        let mut engine = engine_setup(); // Asumo que esto inicializa el CodexEngine

        // 2. El código fuente de prueba (Multiparadigma: Optimización + Lógica)
        let code = r#"
        let global_config = 1;

        optimize "LP" {
            maximize: 2*x
            constraints:
                x <= 10
        }

        boolean "BOOL1" {
            A and B
        }
    "#;

        // 3. Fase de Compilación (Texto -> AST)
        let ast_results = engine.process_file(code);

        // Verificación intermedia (ya confirmaste que esto funciona)
        println!("AST Generado: {:?}", ast_results);
        assert_eq!(ast_results.len(), 2, "Deberíamos tener 2 bloques de instrucciones");

        // 4. Fase de Ejecución (AST -> Routing -> Core)
        // ESTA es la línea crítica que invoca tu nuevo CodexExecutor
        println!("--- Iniciando Ejecución ---");
        CodexExecutor::execute(ast_results);
        println!("--- Ejecución Finalizada ---");
    }
}