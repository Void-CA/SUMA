mod cli;
mod commands;
mod utils; // Si lo creaste, sino omítelo

use clap::Parser;
use anyhow::Result;
use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    // 1. Parsear Argumentos
    let args = Cli::parse();

    // 2. Configurar Logging global (si usaras tracing)
    // if args.verbose { ... }

    // 3. Despachar al Comando correspondiente
    match &args.command {
        Commands::Info => {
            commands::info::execute()?;
        }
        Commands::Run { file } => {
            commands::run::execute(file, args.verbose)?;
        }
        // Aquí agregarás nuevos casos fácilmente:
        // Commands::Repl => commands::repl::execute()?,
    }

    Ok(())
}