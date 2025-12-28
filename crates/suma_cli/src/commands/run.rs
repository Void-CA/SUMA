use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use anyhow::{Context, Result};
use colored::*;

// Imports del Motor
use suma_codex::CodexEngine;
use suma_codex::engine::executor::CodexExecutor;

// Imports de los Parsers (Plugins)
use suma_codex::domains::optimization::parser::OptimizationParser;
use suma_codex::domains::boolean_algebra::parser::BooleanParser;
use suma_codex::domains::linear_algebra::parser::LinearAlgebraParser;

pub fn execute(path: &PathBuf, verbose: bool) -> Result<()> {
    // LOG: Lectura de archivo (Solo verbose)
    if verbose {
        println!(">> Reading file: {:?}", path);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read file '{}'", path.display()))?;

    // 1. Configurar Motor
    let mut engine = CodexEngine::new();
    engine.register(OptimizationParser);
    engine.register(BooleanParser);
    engine.register(LinearAlgebraParser);

    // 2. Parsing
    let start = Instant::now();
    let results = engine.process_file(&content);
    
    // LOG: Tiempos (Solo verbose)
    if verbose {
        let duration = start.elapsed();
        println!(">> Parsing time: {:?}", duration);
    }

    if results.is_empty() {
        // Warning: Esto sí es útil mostrarlo siempre, quizás en amarillo
        println!("{}", "[WARNING] No executable models found in the file.".yellow());
        return Ok(());
    }

    // 3. Ejecución
    if verbose {
        println!("-- Execution Start --");
    }
    
    // Pasamos el flag 'verbose' al ejecutor para que propague el silencio
    CodexExecutor::execute(results, verbose);
    
    if verbose {
        println!("-- Execution End --");
    }

    Ok(())
}