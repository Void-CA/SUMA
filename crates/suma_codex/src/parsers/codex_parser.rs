// src/parsers/codex_parser.rs
use pest_derive::Parser;

#[derive(Parser)]
// La ruta es relativa a la carpeta 'src'
#[grammar = "parsers/codex.pest"] 
pub struct CodexParser;