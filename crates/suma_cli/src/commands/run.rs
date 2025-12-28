use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use anyhow::{Context, Result};
use colored::*;

// Imports del Motor
use suma_codex::CodexEngine;
use suma_codex::engine::executor::CodexExecutor;
use suma_codex::domains::optimization::parser::OptimizationParser;
use suma_codex::domains::boolean_algebra::parser::BooleanParser;
use suma_codex::domains::linear_algebra::parser::LinearAlgebraParser;

pub fn execute(path: &PathBuf, verbose: bool) -> Result<()> {
    if verbose {
        println!("📂 Leyendo archivo: {:?}", path);
    }

    // 1. Leer Archivo
    let content = fs::read_to_string(path)
        .with_context(|| format!("No se pudo leer el archivo '{}'", path.display()))?;

    // 2. Setup del Motor
    let mut engine = CodexEngine::new();

    // Aquí registramos los plugins.
    // EN EL FUTURO: Esto podría leerse de un config file para cargar plugins dinámicos.
    engine.register(OptimizationParser);
    engine.register(BooleanParser);
    engine.register(LinearAlgebraParser);

    // 3. Parsing
    let start = Instant::now();
    let results = engine.process_file(&content);
    let duration = start.elapsed();

    if verbose {
        println!("⏱️  Tiempo de Parsing: {:?}", duration);
    }

    if results.is_empty() {
        println!("{}", "⚠️  No se encontraron bloques ejecutables.".yellow());
        return Ok(());
    }

    // 4. Ejecución
    println!("{}", "🚀 Iniciando ejecución...".bold());
    println!("{}", "-------------------------".dimmed());

    CodexExecutor::execute(results);

    println!("{}", "-------------------------".dimmed());
    println!("{}", "✨ Finalizado.".green());

    Ok(())
}