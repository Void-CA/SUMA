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
use suma_codex::domains::queries::parser::QueryParser; 

use suma_codex::outputs::CodexOutput;

pub fn execute(path: &PathBuf, verbose: bool) -> Result<()> {
    if verbose {
        println!(">> Reading file: {:?}", path);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Could not read file '{}'", path.display()))?;

    // 1. Configurar Motor
    let mut engine = CodexEngine::new();
    
    // Registramos los dominios
    engine.register(OptimizationParser);
    engine.register(BooleanParser);
    engine.register(LinearAlgebraParser);
    engine.register(QueryParser); 

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

    // 3. Ejecución
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
            // Manejamos Message por si acaso (para compatibilidad)
            CodexOutput::Message(msg) => {
                println!();
                println!("{}", msg); // Quitamos italic para que se lea mejor en resultados grandes
            },
            CodexOutput::Error(err) => {
                println!("{}", err.red().bold());
            }
            // Agrega un catch-all por si agregamos nuevos tipos y olvidamos actualizar aquí
            _ => println!("{:?}", output),
        }
    };

    CodexExecutor::execute(results, verbose, &mut console_observer);
    
    if verbose { println!("-- Execution End --"); }

    Ok(())
}