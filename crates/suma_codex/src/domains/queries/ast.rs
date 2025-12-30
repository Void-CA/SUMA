use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct QueryBlock {
    pub target_id: String,
    pub commands: Vec<QueryCommand>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryCommand {
    pub action: String,         // Ej: "solve", "determinant"
    pub alias: Option<String>,  // Ej: Some("det_A")
}