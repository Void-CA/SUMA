use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "suma")]
#[command(about = "Symbolic Unified Mathematical Architecture CLI", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Activar modo verbose (logs detallados)
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ejecuta un script .suma
    Run {
        /// Ruta al archivo fuente
        #[arg(required = true)]
        file: PathBuf,
    },

    /// Información del sistema y módulos
    Info,

    // Futuros comandos escalables:
    // Repl,
    // Check { file: PathBuf },
    // Build { project: PathBuf },
}