use crate::ast::CodexResult;
use crate::engine::executors::ExecutorRegistry;
use crate::outputs::CodexOutput;

pub struct CodexExecutor;

impl CodexExecutor {
    /// Execute parsed blocks using the provided executor registry.
    /// The registry owns all domain executors and dispatches based on result type.
    pub fn execute<F>(
        registry: &mut ExecutorRegistry,
        results: Vec<CodexResult>,
        verbose: bool,
        mut observer: F,
    ) where
        F: FnMut(&str, CodexOutput),
    {
        if verbose {
            println!(">> Executor: Orchestrating {} blocks...", results.len());
        }

        for result in &results {
            match result {
                CodexResult::Query(query) => {
                    if verbose {
                        println!("[QUERY] Broadcasting query for '{}'", query.target_id);
                    }
                    if !registry.execute_query(query, &mut observer) {
                        observer(
                            "Error",
                            CodexOutput::Error(format!(
                                "Identifier '{}' not found in any active domain.",
                                query.target_id
                            )),
                        );
                    }
                }
                _ => {
                    if !registry.execute(result, &mut observer) {
                        observer(
                            "Error",
                            CodexOutput::Error(format!(
                                "Unknown block type: {:?}",
                                std::mem::discriminant(result)
                            )),
                        );
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
    use crate::engine::executors::ExecutorRegistry;
    use crate::CodexEngine;
    use super::*;

    use crate::domains::optimization::parser::OptimizationParser;
    use crate::domains::boolean_algebra::BooleanParser;
    use crate::domains::linear_algebra::parser::LinearAlgebraParser;
    use crate::domains::queries::parser::QueryParser;

    use crate::engine::adapters::linear_algebra::LinearAlgebraExecutor;
    use crate::engine::adapters::optimization::OptimizationExecutor;
    use crate::engine::adapters::boolean_algebra::BooleanExecutor;

    fn engine_setup() -> CodexEngine {
        let mut engine = CodexEngine::new();
        engine.register(OptimizationParser);
        engine.register(BooleanParser);
        engine.register(LinearAlgebraParser);
        engine.register(QueryParser);
        engine
    }

    fn registry_setup(verbose: bool) -> ExecutorRegistry {
        let mut reg = ExecutorRegistry::new();
        reg.register(LinearAlgebraExecutor::new(verbose));
        reg.register(OptimizationExecutor::new(verbose));
        reg.register(BooleanExecutor::new(verbose));
        reg
    }

    fn test_observer(alias: &str, output: CodexOutput) {
        println!("[TEST OUTPUT] {}: {:?}", alias, output);
    }

    #[test]
    fn test_linear_algebra_execution() {
        let engine = engine_setup();
        let mut registry = registry_setup(true);

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

        CodexExecutor::execute(&mut registry, results, true, |alias, output| {
            test_observer(alias, output);
        });
    }

    #[test]
    fn test_optimization_pipeline_full() {
        let engine = engine_setup();
        let mut registry = registry_setup(true);

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

        let mut solved = false;

        CodexExecutor::execute(&mut registry, results, true, |alias, output| {
            println!("[TEST OUT] {}: {:?}", alias, output);

            let txt = match output {
                CodexOutput::Message(s) => s,
                CodexOutput::Error(e) => panic!("Unexpected error: {}", e),
                _ => String::new(),
            };

            if alias == "System" {
                assert!(txt.contains("registrado"), "Definition failed");
            }

            if alias == "Solve Result" {
                solved = true;
                assert!(txt.contains("550"), "Optimum should be 550. Got: \n{}", txt);
                assert!(txt.contains("x") && txt.contains("10"), "x should be 10");
            }
        });

        assert!(solved, "Test finished without solving the problem (never received 'Solve Result')");
    }

    #[test]
    fn test_missing_artifact_error() {
        let engine = engine_setup();
        let mut registry = registry_setup(true);

        let code = r#"
        query "Sistema_Fantasma" {
            solve
        }
        "#;

        println!("\n--- TEST: MISSING ARTIFACT ---");
        let results = engine.process_file(code);

        let mut error_caught = false;
        CodexExecutor::execute(&mut registry, results, true, |alias, output| {
            if let CodexOutput::Error(msg) = output {
                println!("[TEST OK] Error caught: {}: {}", alias, msg);
                error_caught = true;
            }
        });
        assert!(error_caught, "Executor should have reported a 'not found' error");
    }
}
