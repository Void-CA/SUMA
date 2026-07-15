use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use anyhow::{Context, Result};
use colored::*;

use suma_codex::CodexEngine;
use suma_codex::engine::executor::CodexExecutor;

// Domain parsers and executors
use suma_codex::domains::optimization::parser::OptimizationParser;
use suma_codex::domains::boolean_algebra::parser::BooleanParser;
use suma_codex::domains::linear_algebra::parser::LinearAlgebraParser;
use suma_codex::domains::queries::parser::QueryParser; 

use suma_codex::engine::adapters::linear_algebra::LinearAlgebraExecutor;
use suma_codex::engine::adapters::optimization::OptimizationExecutor;
use suma_codex::engine::adapters::boolean_algebra::BooleanExecutor;

use suma_codex::outputs::CodexOutput;

pub fn execute(path: &PathBuf, verbose: bool) -> Result<()> {
    if verbose {
        println!(">> Reading file: {:?}", path);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read file '{}'", path.display()))?;

    // 1. Configure engine — register all domains (parser + executor in one call)
    let mut engine = CodexEngine::new();

    engine.register_domain(OptimizationParser, OptimizationExecutor::new(verbose));
    engine.register_domain(BooleanParser, BooleanExecutor::new(verbose));
    engine.register_domain(LinearAlgebraParser, LinearAlgebraExecutor::new(verbose));
    engine.register_domain(QueryParser, LinearAlgebraExecutor::new(verbose));

    // 2. Parsing
    let start = Instant::now();
    let results = engine.process_file(&content);
    
    if verbose {
        let duration = start.elapsed();
        println!(">> Parsing time: {:?}", duration);
    }

    if results.is_empty() {
        println!("{}", "[WARNING] No executable models found in the file.".yellow());
        return Ok(());
    }

    // 3. Execution — registry lives inside engine.executors
    if verbose { println!("-- Execution Start --"); }
    
    let mut console_observer = |label: &str, output: CodexOutput| {
        print!("➜ {}: ", label.blue().bold());

        match output {
            CodexOutput::LinAlgScalar(val) => {
                println!("{:.4}", val.to_string().green());
            },
            CodexOutput::LinAlgMatrix(mat) | CodexOutput::LinAlgVector(mat) => {
                println!();
                println!("{:.2}", mat);
            },
            CodexOutput::Message(msg) => {
                println!();
                println!("{}", msg);
            },
            CodexOutput::Error(err) => {
                println!("{}", err.red().bold());
            }
            #[allow(unreachable_patterns)]
            _ => println!("{:?}", output),
        }
    };

    CodexExecutor::execute(&mut engine.executors, results, verbose, &mut console_observer);
    
    if verbose { println!("-- Execution End --"); }

    Ok(())
}
